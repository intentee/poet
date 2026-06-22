use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;

use crate::author::Author;
use crate::author_basename::AuthorBasename;
use crate::author_collection::AuthorCollection;
use crate::author_data::AuthorData;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem;
use crate::filesystem::storage::Storage;

pub async fn build_authors(source_filesystem: Arc<Storage>) -> Result<AuthorCollection> {
    let mut authors = AuthorCollection::default();
    let error_collection: DocumentErrorCollection = Default::default();

    for file in source_filesystem.read_project_files().await? {
        if file.kind.is_author() {
            let data: AuthorData = match toml::from_str(&file.contents) {
                Ok(data) => data,
                Err(err) => {
                    error_collection.register_error(
                        file.relative_path.display().to_string(),
                        anyhow!("Failed to parse author file: {err}"),
                    );
                    continue;
                }
            };

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("authors"));
            let basename: AuthorBasename = basename_path.into();

            authors.insert(basename.clone(), Author { basename, data });
        }
    }

    if error_collection.is_empty() {
        Ok(authors)
    } else {
        Err(anyhow!("{error_collection}"))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn builds_author_collection_from_toml_files() -> Result<()> {
        let directory = tempdir()?;
        let storage = Storage {
            base_directory: directory.path().to_path_buf(),
        };

        storage
            .set_file_contents(&PathBuf::from("authors/alice.toml"), "name = \"Alice\"")
            .await?;

        let resolved = build_authors(Arc::new(storage))
            .await?
            .resolve(&["alice".to_string()]);

        assert_eq!(resolved.found_authors.len(), 1);
        assert_eq!(resolved.found_authors[0].data.name, "Alice");

        Ok(())
    }

    #[tokio::test]
    async fn errors_when_an_author_file_cannot_be_parsed() -> Result<()> {
        let directory = tempdir()?;
        let storage = Storage {
            base_directory: directory.path().to_path_buf(),
        };

        storage
            .set_file_contents(&PathBuf::from("authors/broken.toml"), "unexpected = true")
            .await?;

        assert!(build_authors(Arc::new(storage)).await.is_err());

        Ok(())
    }
}
