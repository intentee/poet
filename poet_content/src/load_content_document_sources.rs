use std::sync::Arc;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::filesystem::Filesystem;
use poet_filesystem::source_file::SourceFile;
use poet_mdx::find_front_matter_in_mdast::find_front_matter_in_mdast;
use poet_mdx::string_to_mdast::string_to_mdast;

use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_source::ContentDocumentSource;
use crate::content_error::ContentError;
use crate::content_source_directory::CONTENT_SOURCE_DIRECTORY;

fn content_document_source(
    SourceFile {
        file_entry,
        stem_path,
    }: SourceFile,
    generated_page_base_path: &str,
) -> Result<ContentDocumentSource, ContentError> {
    let FileEntry {
        contents,
        relative_path,
        ..
    } = &file_entry;
    let mdast = string_to_mdast(contents).map_err(|source| ContentError::ParseContentDocument {
        relative_path: relative_path.clone(),
        source,
    })?;
    let front_matter: ContentDocumentFrontMatter = find_front_matter_in_mdast(&mdast)
        .map_err(|source| ContentError::ParseContentDocument {
            relative_path: relative_path.clone(),
            source,
        })?
        .ok_or_else(|| ContentError::MissingFrontMatter {
            relative_path: relative_path.clone(),
        })?;

    Ok(ContentDocumentSource {
        file_entry,
        mdast,
        reference: ContentDocumentReference {
            basename_path: stem_path,
            front_matter: Arc::new(front_matter),
            generated_page_base_path: generated_page_base_path.to_owned(),
        },
    })
}

pub async fn load_content_document_sources<TFilesystem: Filesystem>(
    source_filesystem: &TFilesystem,
    generated_page_base_path: &str,
) -> Result<Vec<ContentDocumentSource>, ContentError> {
    source_filesystem
        .read_source_files(&CONTENT_SOURCE_DIRECTORY)
        .await
        .map_err(ContentError::ReadContentFiles)?
        .into_iter()
        .map(|source_file| content_document_source(source_file, generated_page_base_path))
        .collect()
}
