pub mod build_project_result;
pub mod build_project_result_holder;
mod document_error;
mod document_error_collection;
mod document_rendering_context;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use blake3::hash;
use dashmap::DashMap;
use esbuild_metafile::EsbuildMetaFile;
use log::debug;
use log::info;
use log::warn;
use rayon::prelude::*;
use rhai::Dynamic;
use syntect::parsing::SyntaxSet;

use crate::asset_manager::AssetManager;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::build_project_result::BuildProjectResult;
use crate::build_project::document_error_collection::DocumentErrorCollection;
use crate::build_project::document_rendering_context::DocumentRenderingContext;
use crate::build_timer::BuildTimer;
use crate::component_context::ComponentContext;
use crate::eval_mdast::eval_mdast;
use crate::filesystem::Filesystem as _;
use crate::filesystem::memory::Memory;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::filesystem::storage::Storage;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::find_table_of_contents_in_mdast::find_table_of_contents_in_mdast;
use crate::markdown_document::MarkdownDocument;
use crate::markdown_document_collection::MarkdownDocumentCollection;
use crate::markdown_document_collection_ranked::MarkdownDocumentCollectionRanked;
use crate::markdown_document_in_collection::MarkdownDocumentInCollection;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::markdown_document_source::MarkdownDocumentSource;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::string_to_mdast::string_to_mdast;

fn render_document<'render>(
    DocumentRenderingContext {
        asset_path_renderer,
        available_collections,
        esbuild_metafile,
        is_watching,
        markdown_basename_by_id,
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
        markdown_document_by_basename,
        markdown_document_collections_ranked,
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
        markdown_document_collections_ranked,
        table_of_contents: None,
    };

    let table_of_contents = find_table_of_contents_in_mdast(
        mdast,
        &component_context,
        rhai_template_renderer,
        syntax_set,
    )?;

    let component_context_with_toc = component_context.with_table_of_contents(table_of_contents);

    let layout_content = eval_mdast(
        mdast,
        &component_context_with_toc,
        rhai_template_renderer,
        syntax_set,
    )?;

    rhai_template_renderer.render(
        &front_matter.layout,
        component_context_with_toc.clone(),
        Dynamic::from_map(front_matter.props.clone()),
        layout_content.into(),
    )
}

pub async fn build_project(
    asset_path_renderer: AssetPathRenderer,
    generated_page_base_path: String,
    is_watching: bool,
    rhai_template_renderer: RhaiTemplateRenderer,
    source_filesystem: Arc<Storage>,
) -> Result<BuildProjectResult> {
    info!("Processing content files...");

    let _build_timer = BuildTimer::new();
    let error_collection: DocumentErrorCollection = Default::default();
    let esbuild_metafile: Arc<EsbuildMetaFile> = match source_filesystem
        .read_file_contents(&PathBuf::from("esbuild-meta.json"))
        .await?
    {
        ReadFileContentsResult::Directory => {
            return Err(anyhow!(
                "esbuild metafile should be a file, not a directory"
            ));
        }
        ReadFileContentsResult::Found { contents } => EsbuildMetaFile::from_str(&contents)?,
        ReadFileContentsResult::NotFound => {
            warn!("esbuild metafile not found, proceeding without it");

            EsbuildMetaFile::default()
        }
    }
    .into();
    let files = source_filesystem.read_project_files().await?;
    let memory_filesystem = Arc::new(Memory::default());
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let mut markdown_basename_by_id: HashMap<String, String> = HashMap::new();
    let mut markdown_document_by_basename: HashMap<String, MarkdownDocumentReference> =
        HashMap::new();
    let mut markdown_document_collections: HashMap<String, MarkdownDocumentCollection> =
        HashMap::new();
    let mut markdown_document_collections_ranked: HashMap<
        String,
        MarkdownDocumentCollectionRanked,
    > = HashMap::new();
    let mut markdown_document_list: Vec<MarkdownDocument> = Vec::new();
    let mut markdown_document_sources: BTreeMap<String, MarkdownDocumentSource> =
        Default::default();

    for file in files {
        if file.kind.is_content() {
            let mdast = string_to_mdast(&file.contents)?;
            let front_matter = find_front_matter_in_mdast(&mdast)?.ok_or_else(|| {
                anyhow!("No front matter found in file: {:?}", file.relative_path)
            })?;

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("content"));
            let basename = basename_path.display().to_string();
            let markdown_document_reference = MarkdownDocumentReference {
                basename_path,
                front_matter: front_matter.clone(),
                generated_page_base_path: generated_page_base_path.clone(),
            };

            if let Some(id) = &front_matter.id {
                if markdown_basename_by_id.contains_key(id) {
                    error_collection.register_error(
                        anyhow!("Duplicate document id: #{id} in '{basename}'"),
                        markdown_document_reference.clone(),
                    );
                }

                markdown_basename_by_id.insert(id.clone(), basename.clone());
            }

            markdown_document_by_basename.insert(basename.clone(), markdown_document_reference.clone());
            markdown_document_list.push(MarkdownDocument {
                mdast: mdast.clone(),
                reference: markdown_document_reference.clone(),
            });

            if markdown_document_reference.front_matter.render {
                let relative_path = format!("{basename}.md");

                markdown_document_sources.insert(
                    relative_path.clone(),
                    MarkdownDocumentSource {
                        file_entry: file,
                        mdast,
                        reference: markdown_document_reference,
                        relative_path,
                    },
                );
            }
        }
    }

    // Validate before/after/parent documents in collections
    for reference in markdown_document_by_basename.values() {
        // Validate primary collections
        if let Some(primary_collection) = &reference.front_matter.primary_collection
            && !reference
                .front_matter
                .collections
                .placements
                .iter()
                .any(|placement| placement.name == *primary_collection)
        {
            error_collection.register_error(
                anyhow!(
                    "Document does belong to the collection it claims to be it's primary collection"
                ),
                reference.clone(),
            );
        }

        for collection in &reference.front_matter.collections.placements {
            if let Some(after) = &collection.after
                && !markdown_document_by_basename.contains_key(after)
            {
                error_collection.register_error(
                    anyhow!("Succeeding document does not exist: '{after}'"),
                    reference.clone(),
                );
            }

            if let Some(parent) = &collection.parent
                && !markdown_document_by_basename.contains_key(parent)
            {
                error_collection.register_error(
                    anyhow!("Parent document does not exist: '{parent}'"),
                    reference.clone(),
                );
            }

            markdown_document_collections
                .entry(collection.name.clone())
                .or_insert_with(|| MarkdownDocumentCollection {
                    documents: Default::default(),
                    name: collection.name.clone(),
                })
                .documents
                .push(MarkdownDocumentInCollection {
                    collection_placement: collection.clone(),
                    reference: reference.clone(),
                })
        }
    }

    for markdown_document_collection in markdown_document_collections.values() {
        let markdown_document_collection_ranked: MarkdownDocumentCollectionRanked =
            markdown_document_collection.clone().try_into()?;

        markdown_document_collections_ranked.insert(
            markdown_document_collection.name.clone(),
            markdown_document_collection_ranked,
        );
    }

    if !error_collection.is_empty() {
        return Err(anyhow!("{error_collection}"));
    }

    let available_collections_arc: Arc<HashSet<String>> = Arc::new(
        markdown_document_collections
            .keys()
            .map(|key| key.to_string())
            .collect::<HashSet<String>>(),
    );
    let markdown_document_reference_collection_dashmap: DashMap<String, MarkdownDocumentReference> =
        Default::default();
    let markdown_basename_by_id_arc = Arc::new(markdown_basename_by_id);
    let markdown_document_by_basename_arc = Arc::new(markdown_document_by_basename);
    let markdown_document_collections_ranked_arc = Arc::new(markdown_document_collections_ranked);

    markdown_document_list
        .par_iter()
        .filter(|markdown_document| {
            if !markdown_document.reference.front_matter.render {
                debug!(
                    "Document will not be rendered: {}",
                    markdown_document.reference.basename()
                );

                false
            } else {
                true
            }
        })
        .for_each(|markdown_document| {
            match render_document(DocumentRenderingContext {
                asset_path_renderer: asset_path_renderer.clone(),
                available_collections: available_collections_arc.clone(),
                esbuild_metafile: esbuild_metafile.clone(),
                is_watching,
                markdown_basename_by_id: markdown_basename_by_id_arc.clone(),
                markdown_document,
                markdown_document_by_basename: markdown_document_by_basename_arc.clone(),
                markdown_document_collections_ranked: markdown_document_collections_ranked_arc
                    .clone(),
                rhai_template_renderer: &rhai_template_renderer,
                syntax_set: &syntax_set,
            }) {
                Ok(processed_file) => {
                    match markdown_document.reference.target_file_relative_path() {
                        Ok(relative_path) => {
                            if let Err(err) = memory_filesystem
                                .set_file_contents_sync(&relative_path, &processed_file)
                            {
                                error_collection
                                    .register_error(err, markdown_document.reference.clone());
                            } else {
                                markdown_document_reference_collection_dashmap.insert(
                                    relative_path.display().to_string(),
                                    markdown_document.reference.clone(),
                                );
                            }
                        }
                        Err(err) => {
                            error_collection
                                .register_error(anyhow!(err), markdown_document.reference.clone());
                        }
                    }
                }
                Err(err) => {
                    error_collection.register_error(err, markdown_document.reference.clone())
                }
            }
        });

    if error_collection.is_empty() {
        Ok(BuildProjectResult {
            esbuild_metafile,
            markdown_document_sources: Arc::new(markdown_document_sources),
            memory_filesystem,
        })
    } else {
        Err(anyhow!("{error_collection}"))
    }
}
