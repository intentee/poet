use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;

use crate::author::Author;
use crate::author_basename::AuthorBasename;
use crate::author_front_matter::AuthorFrontMatter;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem;
use crate::filesystem::storage::Storage;

pub async fn build_authors(
    source_filesystem: Arc<Storage>,
) -> Result<BTreeMap<AuthorBasename, Author>> {
    let mut authors: BTreeMap<AuthorBasename, Author> = BTreeMap::new();
    let error_collection: DocumentErrorCollection = Default::default();

    for file in source_filesystem.read_project_files().await? {
        if file.kind.is_author() {
            let front_matter: AuthorFrontMatter = match toml::from_str(&file.contents) {
                Ok(front_matter) => front_matter,
                Err(err) => {
                    error_collection.register_error(
                        file.relative_path.display().to_string(),
                        anyhow!("Failed to parse author file: {err}"),
                    );
                    continue; // skip to next file
                }
            };

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("authors"));
            let basename: AuthorBasename = basename_path.into();

            if authors.contains_key(&basename) {
                error_collection.register_error(
                    format!("author:{}", basename),
                    anyhow!("Duplicate author basename: '{basename}'"),
                );
            } else {
                authors.insert(
                    basename.clone(),
                    Author {
                        basename,
                        front_matter,
                    },
                );
            }
        }
    }

    if error_collection.is_empty() {
        Ok(authors)
    } else {
        Err(anyhow!("{error_collection}"))
    }
}
