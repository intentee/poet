mod document_error;
mod document_error_collection;
mod document_rendering_context;

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use esbuild_metafile::EsbuildMetaFile;
use log::debug;
use log::info;
use log::warn;
use rhai::Dynamic;
use syntect::parsing::SyntaxSet;

use crate::asset_manager::AssetManager;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::document_error::DocumentError;
use crate::build_project::document_error_collection::DocumentErrorCollection;
use crate::build_project::document_rendering_context::DocumentRenderingContext;
use crate::component_context::ComponentContext;
use crate::eval_mdast::eval_mdast;
use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::filesystem::storage::Storage;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::markdown_document::MarkdownDocument;
use crate::markdown_document_collection::MarkdownDocumentCollection;
use crate::markdown_document_in_collection::MarkdownDocumentInCollection;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::rhai_markdown_document_collection::RhaiMarkdownDocumentCollection;
use crate::rhai_template_factory::RhaiTemplateFactory;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::string_to_mdast::string_to_mdast;

async fn render_document<'render>(
    DocumentRenderingContext {
        asset_path_renderer,
        available_collections,
        esbuild_metafile,
        is_watching,
        markdown_basename_by_id,
        markdown_document_by_basename,
        markdown_document:
            MarkdownDocument {
                mdast,
                reference:
                    reference @ MarkdownDocumentReference {
                        basename_path: _,
                        front_matter,
                        generated_page_base_path: _,
                    },
            },
        rhai_markdown_document_collections,
        rhai_template_renderer,
        syntax_set,
    }: DocumentRenderingContext<'render>,
) -> Result<String> {
    let component_context = ComponentContext {
        asset_manager: AssetManager::from_esbuild_metafile(esbuild_metafile, asset_path_renderer),
        available_collections,
        is_watching,
        front_matter: front_matter.clone(),
        markdown_basename_by_id,
        markdown_document_by_basename,
        reference: reference.clone(),
        rhai_markdown_document_collections,
    };

    let layout_content = eval_mdast(
        mdast,
        &component_context,
        rhai_template_renderer,
        syntax_set,
    )?;

    rhai_template_renderer.render(
        &front_matter.layout,
        component_context.clone(),
        Dynamic::from_map(front_matter.props.clone()),
        layout_content.into(),
    )
}

pub async fn build_project(
    asset_path_renderer: AssetPathRenderer,
    generated_page_base_path: String,
    is_watching: bool,
    source_filesystem: &Storage,
) -> Result<Memory> {
    let memory_filesystem = Memory::default();
    let esbuild_metafile: Arc<EsbuildMetaFile> = match source_filesystem
        .read_file_contents(&PathBuf::from("esbuild-meta.json"))
        .await?
    {
        ReadFileContentsResult::Directory => {
            return Err(anyhow!(
                "esbuild metafile should be a file, not a directory"
            ));
        }
        ReadFileContentsResult::Found(contents) => EsbuildMetaFile::from_str(&contents)?,
        ReadFileContentsResult::NotFound => {
            warn!("esbuild metafile not found, proceeding without it");

            EsbuildMetaFile::default()
        }
    }
    .into();

    let files = source_filesystem.read_project_files().await?;
    let rhai_template_factory = RhaiTemplateFactory::new(
        source_filesystem.base_directory.clone(),
        PathBuf::from("shortcodes"),
    );
    let syntax_set = SyntaxSet::load_defaults_newlines();

    for file in &files {
        if file.is_rhai() {
            rhai_template_factory.register_component_file(file.clone());
        }
    }

    info!("Processing shortcodes...");

    let rhai_template_renderer: RhaiTemplateRenderer = rhai_template_factory.try_into()?;

    info!("Processing content files...");

    let mut collections: HashMap<String, MarkdownDocumentCollection> = HashMap::new();
    let mut markdown_basename_by_id: HashMap<String, String> = HashMap::new();
    let mut markdown_document_by_basename: HashMap<String, MarkdownDocumentReference> =
        HashMap::new();
    let mut markdown_document_list: Vec<MarkdownDocument> = Vec::new();

    for file in &files {
        if file.is_markdown() {
            let mdast = string_to_mdast(&file.contents)?;
            let front_matter = find_front_matter_in_mdast(&mdast)?.ok_or_else(|| {
                anyhow!("No front matter found in file: {:?}", file.relative_path)
            })?;

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("content"));
            let basename = basename_path.display().to_string();

            if let Some(id) = &front_matter.id {
                if markdown_basename_by_id.contains_key(id) {
                    return Err(anyhow!("Duplicate document id: #{id} in '{basename}'"));
                }

                markdown_basename_by_id.insert(id.clone(), basename.clone());
            }

            let markdown_document_reference = MarkdownDocumentReference {
                basename_path,
                front_matter: front_matter.clone(),
                generated_page_base_path: generated_page_base_path.clone(),
            };

            for collection in &front_matter.collections.placements {
                collections
                    .entry(collection.name.clone())
                    .or_default()
                    .documents
                    .push(MarkdownDocumentInCollection {
                        collection_placement: collection.clone(),
                        reference: markdown_document_reference.clone(),
                    })
            }

            markdown_document_by_basename.insert(basename, markdown_document_reference.clone());
            markdown_document_list.push(MarkdownDocument {
                mdast,
                reference: markdown_document_reference,
            });
        }
    }

    let available_collections_arc: Arc<HashSet<String>> = Arc::new({
        let mut available_collections: HashSet<String> = Default::default();

        for key in collections.keys() {
            available_collections.insert(key.into());
        }

        available_collections
    });
    let mut error_collection: DocumentErrorCollection = Default::default();
    let markdown_basename_by_id_arc = Arc::new(markdown_basename_by_id);
    let markdown_document_by_basename_arc = Arc::new(markdown_document_by_basename);
    let rhai_markdown_document_collections_arc = Arc::new({
        let mut rhai_markdown_document_collections: HashMap<
            String,
            RhaiMarkdownDocumentCollection,
        > = Default::default();

        for (key, collection) in &collections {
            rhai_markdown_document_collections.insert(
                key.clone(),
                RhaiMarkdownDocumentCollection {
                    available_collections: available_collections_arc.clone(),
                    documents: collection.documents.clone(),
                },
            );
        }

        rhai_markdown_document_collections
    });

    for markdown_document in &markdown_document_list {
        if !markdown_document.reference.front_matter.render {
            debug!(
                "Document will not be rendered: {}",
                markdown_document.reference.basename()
            );
        }

        match render_document(DocumentRenderingContext {
            asset_path_renderer: asset_path_renderer.clone(),
            available_collections: available_collections_arc.clone(),
            esbuild_metafile: esbuild_metafile.clone(),
            is_watching,
            markdown_basename_by_id: markdown_basename_by_id_arc.clone(),
            markdown_document,
            markdown_document_by_basename: markdown_document_by_basename_arc.clone(),
            rhai_markdown_document_collections: rhai_markdown_document_collections_arc.clone(),
            rhai_template_renderer: &rhai_template_renderer,
            syntax_set: &syntax_set,
        })
        .await
        {
            Ok(processed_file) => {
                memory_filesystem
                    .set_file_contents(
                        &match markdown_document.reference.target_file_relative_path() {
                            Ok(relative_path) => relative_path,
                            Err(err) => return Err(anyhow!(err)),
                        },
                        &processed_file,
                    )
                    .await?;
            }
            Err(err) => {
                error_collection
                    .errors
                    .entry(markdown_document.reference.clone())
                    .or_default()
                    .push(DocumentError {
                        err,
                        markdown_document_reference: markdown_document.reference.clone(),
                    });
            }
        }
    }

    if error_collection.errors.is_empty() {
        Ok(memory_filesystem)
    } else {
        error_collection.render();

        Err(anyhow!("failed due to the previous errors"))
    }
}
