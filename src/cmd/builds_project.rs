use std::path::PathBuf;
use std::sync::Arc;

use crate::filesystem::storage::Storage;

pub trait BuildsProject {
    fn source_directory(&self) -> PathBuf;

    fn assets_directory(&self) -> PathBuf {
        let mut static_files_directory: PathBuf = self.source_directory().clone();

        static_files_directory.push("assets");

        static_files_directory
    }

    fn source_filesystem(&self) -> Arc<Storage> {
        Arc::new(Storage {
            base_directory: self.source_directory(),
        })
    }
}
