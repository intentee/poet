use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use dashmap::DashMap;

use super::Filesystem;
use super::file_entry::FileEntry;
use super::read_file_contents_result::ReadFileContentsResult;

pub struct Memory {
    files: DashMap<String, String>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            files: DashMap::new(),
        }
    }
}

#[async_trait]
impl Filesystem for Memory {
    async fn read_project_files(&self) -> Result<Vec<FileEntry>> {
        let file_entries = self
            .files
            .iter()
            .map(|entry| FileEntry {
                contents: entry.value().clone(),
                relative_path: entry.key().into(),
            })
            .collect();

        Ok(file_entries)
    }

    async fn read_file_contents(&self, relative_path: &Path) -> Result<ReadFileContentsResult> {
        let path_str = relative_path
            .to_str()
            .ok_or_else(|| anyhow!("Unable to stringify path"))?;

        if let Some(contents) = self.files.get(path_str) {
            Ok(ReadFileContentsResult::Found(contents.value().to_owned()))
        } else {
            Ok(ReadFileContentsResult::NotFound)
        }
    }

    fn set_file_contents_sync(&self, path: &Path, contents: &str) -> Result<()> {
        self.files.insert(
            path.to_str()
                .ok_or_else(|| anyhow!("Unable to stringify path"))?
                .to_string(),
            contents.to_string(),
        );

        Ok(())
    }
}
