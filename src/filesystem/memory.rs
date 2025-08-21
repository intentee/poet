use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use dashmap::DashMap;

use super::Filesystem;
use super::file_entry::FileEntry;

pub struct Memory {
    files: DashMap<String, String>,
}

#[async_trait]
impl Filesystem for Memory {
    async fn read_all_files(&self) -> Result<Vec<FileEntry>> {
        let file_entries = self
            .files
            .iter()
            .map(|entry| FileEntry {
                contents: entry.value().clone(),
                path: entry.key().into(),
            })
            .collect();

        Ok(file_entries)
    }

    async fn set_file_contents(&self, path: &Path, contents: &str) -> Result<()> {
        self.files.insert(
            path.to_str()
                .ok_or_else(|| anyhow!("Unable to stringify path"))?
                .to_string(),
            contents.to_string(),
        );

        Ok(())
    }
}
