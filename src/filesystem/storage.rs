use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use tokio::fs;

use super::Filesystem;
use super::file_entry::FileEntry;

pub struct Storage {
    pub base_directory: PathBuf,
}

#[async_trait]
impl Filesystem for Storage {
    async fn read_all_files(&self) -> Result<Vec<FileEntry>> {
        let mut to_visit = vec![self.base_directory.clone()];
        let mut files = Vec::new();

        while let Some(current) = to_visit.pop() {
            let mut entries = fs::read_dir(current).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    to_visit.push(path);
                } else {
                    let relative_path = path.strip_prefix(&self.base_directory)?.to_path_buf();

                    files.push(FileEntry {
                        contents: fs::read_to_string(&path).await?,
                        path: relative_path,
                    });
                }
            }
        }

        Ok(files)
    }

    async fn set_file_contents(&self, relative_path: &Path, contents: &str) -> Result<()> {
        fs::write(&self.base_directory.join(relative_path), contents).await?;

        Ok(())
    }
}
