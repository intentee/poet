use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;

use crate::build_content_document_sources_result::BuildContentDocumentSourcesResult;
use crate::content_document::ContentDocument;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_source::ContentDocumentSource;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem as _;
use crate::filesystem::storage::Storage;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::string_to_mdast::string_to_mdast;

pub async fn build_content_document_sources(
    source_filesystem: &Arc<Storage>,
    generated_page_base_path: &str,
) -> Result<BuildContentDocumentSourcesResult> {
    let error_collection: DocumentErrorCollection = Default::default();
    let mut content_document_basename_by_id: HashMap<String, ContentDocumentBasename> =
        HashMap::new();
    let mut content_document_by_basename: HashMap<
        ContentDocumentBasename,
        ContentDocumentReference,
    > = HashMap::new();
    let mut content_document_list: Vec<ContentDocument> = Vec::new();
    let mut content_document_sources: BTreeMap<ContentDocumentBasename, ContentDocumentSource> =
        Default::default();

    for file in source_filesystem.read_project_files().await? {
        if file.kind.is_content() {
            let mdast = string_to_mdast(&file.contents)?;
            let front_matter: ContentDocumentFrontMatter = find_front_matter_in_mdast(&mdast)?
                .ok_or_else(|| {
                    anyhow!("No front matter found in file: {:?}", file.relative_path)
                })?;

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("content"));
            let basename: ContentDocumentBasename = basename_path.clone().into();
            let content_document_reference = ContentDocumentReference {
                basename_path,
                front_matter: front_matter.clone(),
                generated_page_base_path: generated_page_base_path.to_string(),
            };

            if let Some(id) = &front_matter.id {
                if content_document_basename_by_id.contains_key(id) {
                    error_collection.register_error(
                        content_document_reference.basename().to_string(),
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

    if !error_collection.is_empty() {
        return Err(anyhow!("{error_collection}"));
    }

    Ok(BuildContentDocumentSourcesResult {
        content_document_basename_by_id,
        content_document_by_basename,
        content_document_list,
        content_document_sources,
    })
}
