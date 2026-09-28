use std::path::PathBuf;
use std::sync::Arc;

use poet_filesystem::storage::Storage;

use crate::cmd::STATIC_FILES_PUBLIC_PATH;

pub trait BuildsProject {
    fn source_directory(&self) -> PathBuf;

    fn assets_directory(&self) -> PathBuf {
        self.source_directory().join(STATIC_FILES_PUBLIC_PATH)
    }

    fn source_filesystem(&self) -> Arc<Storage> {
        Arc::new(Storage {
            base_directory: self.source_directory(),
        })
    }
}
