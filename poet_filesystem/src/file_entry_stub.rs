use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FileEntryStub {
    pub contents: String,
    pub relative_path: PathBuf,
}
