use std::io;

use poet_filesystem::storage::Storage;
use tempfile::TempDir;
use tempfile::tempdir;

pub struct TemporaryStorage {
    pub directory: TempDir,
    pub storage: Storage,
}

impl TemporaryStorage {
    pub fn create() -> io::Result<Self> {
        let directory = tempdir()?;

        Ok(Self {
            storage: Storage {
                base_directory: directory.path().to_path_buf(),
            },
            directory,
        })
    }
}
