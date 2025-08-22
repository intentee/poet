use std::path::PathBuf;

#[derive(Debug)]
pub struct FileEntry {
    pub contents: String,
    pub path: PathBuf,
}

impl FileEntry {
    pub fn has_extension(&self, extension: &str) -> bool {
        self.path
            .extension()
            .map(|ext| ext == extension)
            .unwrap_or(false)
    }

    pub fn is_markdown(&self) -> bool {
        self.has_extension("md")
    }

    pub fn is_rhai(&self) -> bool {
        self.has_extension("rhai")
    }
}
