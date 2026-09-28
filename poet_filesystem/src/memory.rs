use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::file_entry::FileEntry;
use crate::file_entry_stub::FileEntryStub;
use crate::filesystem::Filesystem;
use crate::filesystem_error::FilesystemError;
use crate::read_file_contents_result::ReadFileContentsResult;
use crate::source_directory::SourceDirectory;
use crate::source_file::SourceFile;

#[derive(Default)]
pub struct Memory {
    files: DashMap<PathBuf, String>,
}

impl Memory {
    pub async fn copy_all_files_to<TFilesystem: Filesystem>(
        &self,
        target_filesystem: &TFilesystem,
    ) -> Result<(), FilesystemError> {
        for FileEntry {
            contents,
            relative_path,
            ..
        } in self.file_entries()
        {
            target_filesystem
                .set_file_contents(&relative_path, &contents)
                .await?;
        }

        Ok(())
    }

    #[must_use]
    pub fn file_entries(&self) -> Vec<FileEntry> {
        self.files
            .iter()
            .map(|stored_file| {
                FileEntry::from(FileEntryStub {
                    contents: stored_file.value().clone(),
                    relative_path: stored_file.key().clone(),
                })
            })
            .collect()
    }

    pub fn set_file_contents_sync(&self, relative_path: &Path, contents: &str) {
        self.files
            .insert(relative_path.to_path_buf(), contents.to_owned());
    }
}

#[async_trait]
impl Filesystem for Memory {
    async fn read_source_files(
        &self,
        SourceDirectory {
            file_extension,
            name,
        }: &SourceDirectory,
    ) -> Result<Vec<SourceFile>, FilesystemError> {
        Ok(self
            .file_entries()
            .into_iter()
            .filter(|FileEntry { relative_path, .. }| {
                relative_path.extension() == Some(OsStr::new(file_extension))
            })
            .filter_map(|file_entry| {
                let stem_path = file_entry
                    .relative_path
                    .strip_prefix(name)
                    .ok()?
                    .with_extension("");

                Some(SourceFile {
                    file_entry,
                    stem_path,
                })
            })
            .collect())
    }

    async fn read_file_contents(
        &self,
        relative_path: &Path,
    ) -> Result<ReadFileContentsResult, FilesystemError> {
        Ok(self
            .files
            .get(relative_path)
            .map_or(ReadFileContentsResult::NotFound, |stored_file| {
                ReadFileContentsResult::Found {
                    contents: stored_file.value().clone(),
                }
            }))
    }

    async fn set_file_contents(
        &self,
        relative_path: &Path,
        contents: &str,
    ) -> Result<(), FilesystemError> {
        self.set_file_contents_sync(relative_path, contents);

        Ok(())
    }
}
