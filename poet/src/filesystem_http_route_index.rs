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

    fn register_file(&self, file: FileEntry) -> Result<()> {
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
        } else if filename == "sitemap.xml" {
            self.routes.insert(filename, file.clone());
        } else {
            return Err(anyhow!("Unexpected filename: '{filename}'"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::filesystem::file_entry_stub::FileEntryStub;

    fn file_entry(relative_path: &str) -> Result<FileEntry> {
        FileEntryStub {
            contents: String::new(),
            relative_path: PathBuf::from(relative_path),
        }
        .try_into()
    }

    #[test]
    fn registers_root_index_under_empty_and_named_routes() -> Result<()> {
        let index = FilesystemHttpRouteIndex::default();

        index.register_file(file_entry("index.html")?)?;

        assert_eq!(
            index
                .get_file_entry_for_path("")
                .map(|entry| entry.relative_path),
            Some(PathBuf::from("index.html"))
        );
        assert!(index.get_file_entry_for_path("index.html").is_some());

        Ok(())
    }

    #[test]
    fn registers_nested_index_under_directory_and_full_path() -> Result<()> {
        let index = FilesystemHttpRouteIndex::default();

        index.register_file(file_entry("docs/index.html")?)?;

        assert!(index.get_file_entry_for_path("docs/index.html").is_some());
        assert!(index.get_file_entry_for_path("docs/").is_some());

        Ok(())
    }

    #[test]
    fn registers_sitemap_under_its_own_path() -> Result<()> {
        let index = FilesystemHttpRouteIndex::default();

        index.register_file(file_entry("sitemap.xml")?)?;

        assert!(index.get_file_entry_for_path("sitemap.xml").is_some());

        Ok(())
    }

    #[test]
    fn rejects_unexpected_filename() -> Result<()> {
        let index = FilesystemHttpRouteIndex::default();

        assert!(index.register_file(file_entry("page.html")?).is_err());

        Ok(())
    }

    #[test]
    fn returns_none_for_unregistered_path() {
        let index = FilesystemHttpRouteIndex::default();

        assert!(index.get_file_entry_for_path("missing").is_none());
    }
}
