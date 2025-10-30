use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;

use crate::filesystem::Filesystem;
use crate::filesystem::file_entry::FileEntry;

#[derive(Default)]
pub struct FilesystemHttpRouteIndex {
    routes: DashMap<String, FileEntry>,
}

impl FilesystemHttpRouteIndex {
    pub fn get_file_entry_for_path(&self, path: &str) -> Option<FileEntry> {
        self.routes.get(path).map(|entry| entry.value().clone())
    }

    pub async fn from_filesystem<TFilesystem: Filesystem>(
        filesystem: Arc<TFilesystem>,
    ) -> Result<Self> {
        let this: Self = Default::default();

        for file in filesystem.read_project_files().await? {
            this.register_file(file)?;
        }

        Ok(this)
    }

    pub fn register_file(&self, file: FileEntry) -> Result<()> {
        let filename = file.relative_path.to_string_lossy().to_string();

        if filename.ends_with("/index.html") {
            let filename_stripped: String = filename
                .strip_suffix("index.html")
                .ok_or_else(|| anyhow!("Unable to strip '/index.html' suffix from: '{filename}'"))?
                .to_string();

            self.routes.insert(filename, file.clone());
            self.routes.insert(filename_stripped, file.clone());
        } else if filename == "index.html" {
            self.routes.insert("".to_string(), file.clone());
            self.routes.insert("index.html".to_string(), file.clone());
        } else {
            return Err(anyhow!("Unexpected filename: '{filename}'"));
        }

        Ok(())
    }
}
