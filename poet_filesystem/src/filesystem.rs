use std::path::Path;
use std::path::PathBuf;

use async_trait::async_trait;

use crate::file_entry::FileEntry;
use crate::filesystem_error::FilesystemError;
use crate::read_file_contents_result::ReadFileContentsResult;
use crate::source_directory::SourceDirectory;

#[async_trait]
pub trait Filesystem: Send + Sync {
    async fn read_source_files(
        &self,
        source_directory: &SourceDirectory,
    ) -> Result<Vec<FileEntry>, FilesystemError>;

    async fn read_file_contents(
        &self,
        path: &Path,
    ) -> Result<ReadFileContentsResult, FilesystemError>;

    async fn set_file_contents(&self, path: &Path, contents: &str) -> Result<(), FilesystemError>;

    async fn copy_file_from<TFilesystem: Filesystem>(
        &self,
        other: &TFilesystem,
        path: &Path,
    ) -> Result<(), FilesystemError> {
        let other_file_contents = other.read_file_contents_string(path).await?;

        self.set_file_contents(path, &other_file_contents).await
    }

    async fn copy_source_files_from<TFilesystem: Filesystem>(
        &self,
        other: &TFilesystem,
        source_directory: &SourceDirectory,
    ) -> Result<(), FilesystemError> {
        for FileEntry {
            contents,
            relative_path,
            ..
        } in other.read_source_files(source_directory).await?
        {
            self.set_file_contents(&relative_path, &contents).await?;
        }

        Ok(())
    }

    async fn read_file_contents_string(&self, path: &Path) -> Result<String, FilesystemError> {
        match self.read_file_contents(path).await? {
            ReadFileContentsResult::Directory => Err(FilesystemError::FileIsDirectory {
                path: PathBuf::from(path),
            }),
            ReadFileContentsResult::Found { contents } => Ok(contents),
            ReadFileContentsResult::NotFound => Err(FilesystemError::FileNotFound {
                path: PathBuf::from(path),
            }),
        }
    }
}
