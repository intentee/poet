use std::path::PathBuf;

#[derive(Debug)]
pub struct FileEntry {
    pub contents: String,
    pub path: PathBuf,
}
