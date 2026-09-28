use std::path::PathBuf;

use crate::file_entry::FileEntry;
use crate::filesystem_error::FilesystemError;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub file_entry: FileEntry,
    pub stem_path: PathBuf,
}

impl SourceFile {
    pub fn stem_name(&self) -> Result<String, FilesystemError> {
        self.stem_path
            .to_str()
            .map(str::to_owned)
            .ok_or_else(|| FilesystemError::NonUtf8Path {
                path: self.stem_path.clone(),
            })
    }
}
