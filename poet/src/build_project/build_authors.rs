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

    for file in source_filesystem.read_author_files().await? {
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

    if error_collection.is_empty() {
        Ok(authors)
    } else {
        Err(anyhow!("{error_collection}"))
    }
}
