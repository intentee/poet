use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use async_trait::async_trait;
use tokio::fs;

use super::Filesystem;
use super::file_entry::FileEntry;
use super::read_file_contents_result::ReadFileContentsResult;

pub struct Storage {
    pub base_directory: PathBuf,
}

#[async_trait]
impl Filesystem for Storage {
    async fn read_project_files(&self) -> Result<Vec<FileEntry>> {
        let mut to_visit: Vec<PathBuf> = vec![
            self.base_directory.join("content"),
            self.base_directory.join("shortcodes"),
        ];
        let mut files = Vec::new();

        while let Some(current) = to_visit.pop() {
            if !current.exists() {
                continue;
            }

            let mut entries = fs::read_dir(current).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    to_visit.push(path);
                } else {
                    let relative_path = path.strip_prefix(&self.base_directory)?.to_path_buf();

                    files.push(FileEntry {
                        contents: fs::read_to_string(&path)
                            .await
                            .context(format!("Failed to read file: {}", path.display()))?,
                        relative_path,
                    });
                }
            }
        }

        Ok(files)
    }

    async fn read_file_contents(&self, relative_path: &Path) -> Result<ReadFileContentsResult> {
        let full_path = self.base_directory.join(relative_path);

        if !full_path.exists() {
            return Ok(ReadFileContentsResult::NotFound);
        }

        if full_path.is_dir() {
            return Ok(ReadFileContentsResult::Directory);
        }

        let contents = fs::read_to_string(&full_path).await?;

        Ok(ReadFileContentsResult::Found(contents))
    }

    async fn set_file_contents(&self, relative_path: &Path, contents: &str) -> Result<()> {
        let full_path = self.base_directory.join(relative_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(&full_path, contents)
            .await
            .context(format!("Failed to write file: {}", full_path.display()))?;

        Ok(())
    }
}
