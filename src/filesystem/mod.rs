mod file_entry;
pub mod memory;
pub mod storage;

use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;

use self::file_entry::FileEntry;

#[async_trait]
pub trait Filesystem {
    async fn read_all_files(&self) -> Result<Vec<FileEntry>>;

    async fn set_file_contents(&self, path: &Path, content: &str) -> Result<()>;
}
