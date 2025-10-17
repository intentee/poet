pub mod build_project_result;
pub mod build_project_result_holder;
pub mod build_project_result_stub;
mod content_document_error;
mod content_document_error_collection;
mod content_document_rendering_context;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
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
use crate::build_project::build_project_result_stub::BuildProjectResultStub;
use crate::build_project::content_document_error_collection::ContentDocumentErrorCollection;
use crate::build_project::content_document_rendering_context::ContentDocumentRenderingContext;
use crate::build_timer::BuildTimer;
use crate::component_context::ComponentContext;
use crate::content_document::ContentDocument;
use crate::content_document_collection::ContentDocumentCollection;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_in_collection::ContentDocumentInCollection;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_source::ContentDocumentSource;
use crate::eval_mdast::eval_mdast;
use crate::filesystem::Filesystem as _;
use crate::filesystem::memory::Memory;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::filesystem::storage::Storage;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::find_table_of_contents_in_mdast::find_table_of_contents_in_mdast;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::string_to_mdast::string_to_mdast;

fn render_document<'render>(
    ContentDocumentRenderingContext {
        asset_path_renderer,
        available_collections,
        content_document:
            ContentDocument {
                mdast,
                reference:
                    reference @ ContentDocumentReference {
                        basename_path: _,
                        front_matter,
                        generated_page_base_path: _,
                    },
            },
        content_document_basename_by_id,
        content_document_by_basename,
        content_document_collections_ranked,
        esbuild_metafile,
        is_watching,
        rhai_template_renderer,
        syntax_set,
    }: ContentDocumentRenderingContext<'render>,
) -> Result<String> {
    let component_context = ComponentContext {
        asset_manager: AssetManager::from_esbuild_metafile(esbuild_metafile, asset_path_renderer),
        available_collections,
        content_document_basename_by_id,
        content_document_by_basename,
        content_document_collections_ranked,
        front_matter: front_matter.clone(),
        is_watching,
        reference: reference.clone(),
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
) -> Result<BuildProjectResultStub> {
    info!("Processing content files...");

    let _build_timer = BuildTimer::new();
    let error_collection: ContentDocumentErrorCollection = Default::default();
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

    let mut content_document_basename_by_id: HashMap<String, String> = HashMap::new();
    let mut content_document_by_basename: HashMap<String, ContentDocumentReference> =
        HashMap::new();
    let mut content_document_collections: HashMap<String, ContentDocumentCollection> =
        HashMap::new();
    let mut content_document_collections_ranked: HashMap<String, ContentDocumentCollectionRanked> =
        HashMap::new();
    let mut content_document_list: Vec<ContentDocument> = Vec::new();
    let mut content_document_sources: BTreeMap<String, ContentDocumentSource> = Default::default();

    for file in files {
        if file.kind.is_content() {
            let mdast = string_to_mdast(&file.contents)?;
            let front_matter = find_front_matter_in_mdast(&mdast)?.ok_or_else(|| {
                anyhow!("No front matter found in file: {:?}", file.relative_path)
            })?;

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("content"));
            let basename = basename_path.display().to_string();
            let content_document_reference = ContentDocumentReference {
                basename_path,
                front_matter: front_matter.clone(),
                generated_page_base_path: generated_page_base_path.clone(),
            };

            if let Some(id) = &front_matter.id {
                if content_document_basename_by_id.contains_key(id) {
                    error_collection.register_error(
                        content_document_reference.clone(),
                        anyhow!("Duplicate document id: #{id} in '{basename}'"),
                    );
                }

                content_document_basename_by_id.insert(id.clone(), basename.clone());
            }

            content_document_by_basename
                .insert(basename.clone(), content_document_reference.clone());
            content_document_list.push(ContentDocument {
                mdast: mdast.clone(),
                reference: content_document_reference.clone(),
            });

            if content_document_reference.front_matter.render {
                let relative_path = format!("{basename}.md");

                content_document_sources.insert(
                    basename,
                    ContentDocumentSource {
                        file_entry: file,
                        mdast,
                        reference: content_document_reference,
                        relative_path,
                    },
                );
            }
        }
    }

    // Validate before/after/parent documents in collections
    for reference in content_document_by_basename.values() {
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
                reference.clone(),
                anyhow!(
                    "Document does belong to the collection it claims to be it's primary collection"
                ),
            );
        }

        for collection in &reference.front_matter.collections.placements {
            if let Some(after) = &collection.after
                && !content_document_by_basename.contains_key(after)
            {
                error_collection.register_error(
                    reference.clone(),
                    anyhow!("Succeeding document does not exist: '{after}'"),
                );
            }

            if let Some(parent) = &collection.parent
                && !content_document_by_basename.contains_key(parent)
            {
                error_collection.register_error(
                    reference.clone(),
                    anyhow!("Parent document does not exist: '{parent}'"),
                );
            }

            content_document_collections
                .entry(collection.name.clone())
                .or_insert_with(|| ContentDocumentCollection {
                    documents: Default::default(),
                    name: collection.name.clone(),
                })
                .documents
                .push(ContentDocumentInCollection {
                    collection_placement: collection.clone(),
                    reference: reference.clone(),
                })
        }
    }

    for content_document_collection in content_document_collections.values() {
        let content_document_collection_ranked: ContentDocumentCollectionRanked =
            content_document_collection.clone().try_into()?;

        content_document_collections_ranked.insert(
            content_document_collection.name.clone(),
            content_document_collection_ranked,
        );
    }

    if !error_collection.is_empty() {
        return Err(anyhow!("{error_collection}"));
    }

    let available_collections_arc: Arc<HashSet<String>> = Arc::new(
        content_document_collections
            .keys()
            .map(|key| key.to_string())
            .collect::<HashSet<String>>(),
    );
    let content_document_reference_collection_dashmap: DashMap<String, ContentDocumentReference> =
        Default::default();
    let content_document_basename_by_id_arc = Arc::new(content_document_basename_by_id);
    let content_document_by_basename_arc = Arc::new(content_document_by_basename);
    let content_document_collections_ranked_arc = Arc::new(content_document_collections_ranked);

    content_document_list
        .par_iter()
        .filter(|content_document| {
            if !content_document.reference.front_matter.render {
                debug!(
                    "Document will not be rendered: {}",
                    content_document.reference.basename()
                );

                false
            } else {
                true
            }
        })
        .for_each(|content_document| {
            match render_document(ContentDocumentRenderingContext {
                asset_path_renderer: asset_path_renderer.clone(),
                available_collections: available_collections_arc.clone(),
                esbuild_metafile: esbuild_metafile.clone(),
                is_watching,
                content_document_basename_by_id: content_document_basename_by_id_arc.clone(),
                content_document,
                content_document_by_basename: content_document_by_basename_arc.clone(),
                content_document_collections_ranked: content_document_collections_ranked_arc
                    .clone(),
                rhai_template_renderer: &rhai_template_renderer,
                syntax_set: &syntax_set,
            }) {
                Ok(processed_file) => {
                    match content_document.reference.target_file_relative_path() {
                        Ok(relative_path) => {
                            if let Err(err) = memory_filesystem
                                .set_file_contents_sync(&relative_path, &processed_file)
                            {
                                error_collection
                                    .register_error(content_document.reference.clone(), err);
                            } else {
                                content_document_reference_collection_dashmap.insert(
                                    relative_path.display().to_string(),
                                    content_document.reference.clone(),
                                );
                            }
                        }
                        Err(err) => {
                            error_collection
                                .register_error(content_document.reference.clone(), anyhow!(err));
                        }
                    }
                }
                Err(err) => {
                    error_collection.register_error(content_document.reference.clone(), err)
                }
            }
        });

    if error_collection.is_empty() {
        Ok(BuildProjectResultStub {
            esbuild_metafile,
            content_document_sources: Arc::new(content_document_sources),
            memory_filesystem,
        })
    } else {
        Err(anyhow!("{error_collection}"))
    }
}
