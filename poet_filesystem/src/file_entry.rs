use std::path::PathBuf;

use blake3::Hash;
use blake3::hash;

use crate::file_entry_stub::FileEntryStub;

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub contents: String,
    pub contents_hash: Hash,
    pub relative_path: PathBuf,
}

impl From<FileEntryStub> for FileEntry {
    fn from(
        FileEntryStub {
            contents,
            relative_path,
        }: FileEntryStub,
    ) -> Self {
        Self {
            contents_hash: hash(contents.as_bytes()),
            contents,
            relative_path,
        }
    }
}
