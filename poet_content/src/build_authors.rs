use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::filesystem::Filesystem;
use poet_filesystem::source_file::SourceFile;
use poet_mdx::document_error_collection::DocumentErrorCollection;

use crate::author::Author;
use crate::author_basename::AuthorBasename;
use crate::author_collection::AuthorCollection;
use crate::authors_source_directory::AUTHORS_SOURCE_DIRECTORY;
use crate::content_error::ContentError;

pub async fn build_authors<TFilesystem: Filesystem>(
    source_filesystem: &TFilesystem,
) -> Result<AuthorCollection, ContentError> {
    let mut author_collection = AuthorCollection::default();
    let author_errors = DocumentErrorCollection::default();

    for SourceFile {
        file_entry:
            FileEntry {
                contents,
                relative_path,
                ..
            },
        stem_path,
    } in source_filesystem
        .read_source_files(&AUTHORS_SOURCE_DIRECTORY)
        .await
        .map_err(ContentError::ReadAuthorFiles)?
    {
        match toml::from_str(&contents) {
            Ok(data) => author_collection.insert(Author {
                basename: AuthorBasename::from(stem_path.as_path()),
                data,
            }),
            Err(source) => author_errors.register_error(
                relative_path.display().to_string(),
                ContentError::ParseAuthor {
                    relative_path,
                    source,
                },
            ),
        }
    }

    if author_errors.is_empty() {
        Ok(author_collection)
    } else {
        Err(ContentError::InvalidAuthors(author_errors))
    }
}
