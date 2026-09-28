use std::path::PathBuf;

use blake3::Hash;
use blake3::hash;

use crate::file_entry_stub::FileEntryStub;
use crate::filesystem_error::FilesystemError;
use crate::source_directory::SourceDirectory;

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub contents: String,
    pub contents_hash: Hash,
    pub relative_path: PathBuf,
}

impl FileEntry {
    pub fn stem_path_in(
        &self,
        SourceDirectory { name, .. }: &SourceDirectory,
    ) -> Result<PathBuf, FilesystemError> {
        self.relative_path
            .strip_prefix(name)
            .map(|path_in_directory| path_in_directory.with_extension(""))
            .map_err(|source| FilesystemError::FileOutsideDirectory {
                directory: PathBuf::from(name),
                relative_path: self.relative_path.clone(),
                source,
            })
    }

    pub fn stem_in(&self, source_directory: &SourceDirectory) -> Result<String, FilesystemError> {
        let stem_path = self.stem_path_in(source_directory)?;

        stem_path
            .to_str()
            .map(str::to_owned)
            .ok_or(FilesystemError::NonUtf8Path { path: stem_path })
    }
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
