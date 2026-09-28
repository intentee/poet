use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::read_dir;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use async_trait::async_trait;
use tokio::fs::metadata;
use tokio::fs::read_to_string;
use tokio::fs::write;

use crate::create_parent_directories::create_parent_directories;
use crate::file_entry::FileEntry;
use crate::file_entry_stub::FileEntryStub;
use crate::filesystem::Filesystem;
use crate::filesystem_error::FilesystemError;
use crate::read_file_contents_result::ReadFileContentsResult;
use crate::source_directory::SourceDirectory;

struct DirectoryEntry {
    file_name: OsString,
    is_directory: bool,
}

fn read_directory_entries(directory_path: &Path) -> io::Result<Vec<DirectoryEntry>> {
    read_dir(directory_path).and_then(|directory_entries| {
        directory_entries
            .map(|directory_entry| {
                directory_entry.and_then(|directory_entry| {
                    directory_entry.file_type().map(|file_type| DirectoryEntry {
                        file_name: directory_entry.file_name(),
                        is_directory: file_type.is_dir(),
                    })
                })
            })
            .collect()
    })
}

pub struct Storage {
    pub base_directory: PathBuf,
}

impl Storage {
    fn source_directory_entries(
        &self,
        relative_directory: &Path,
    ) -> Result<Vec<DirectoryEntry>, FilesystemError> {
        let directory_path = self.base_directory.join(relative_directory);

        match read_directory_entries(&directory_path) {
            Ok(directory_entries) => Ok(directory_entries),
            Err(source) if source.kind() == ErrorKind::NotFound => Ok(vec![]),
            Err(source) => Err(FilesystemError::ReadDirectory {
                path: directory_path,
                source,
            }),
        }
    }
}

#[async_trait]
impl Filesystem for Storage {
    async fn read_source_files(
        &self,
        SourceDirectory {
            file_extension,
            name,
        }: &SourceDirectory,
    ) -> Result<Vec<FileEntry>, FilesystemError> {
        let mut directories_to_visit: Vec<PathBuf> = vec![PathBuf::from(name)];
        let mut file_entries: Vec<FileEntry> = vec![];

        while let Some(relative_directory) = directories_to_visit.pop() {
            for DirectoryEntry {
                file_name,
                is_directory,
            } in self.source_directory_entries(&relative_directory)?
            {
                let relative_path = relative_directory.join(file_name);

                if is_directory {
                    directories_to_visit.push(relative_path);
                } else if relative_path.extension() == Some(OsStr::new(file_extension)) {
                    file_entries.push(FileEntry::from(FileEntryStub {
                        contents: self.read_file_contents_string(&relative_path).await?,
                        relative_path,
                    }));
                }
            }
        }

        Ok(file_entries)
    }

    async fn read_file_contents(
        &self,
        relative_path: &Path,
    ) -> Result<ReadFileContentsResult, FilesystemError> {
        let full_path = self.base_directory.join(relative_path);

        match metadata(&full_path).await {
            Ok(file_metadata) if file_metadata.is_dir() => Ok(ReadFileContentsResult::Directory),
            Ok(_) => read_to_string(&full_path)
                .await
                .map(|contents| ReadFileContentsResult::Found { contents })
                .map_err(|source| FilesystemError::ReadFile {
                    path: full_path,
                    source,
                }),
            Err(source) if source.kind() == ErrorKind::NotFound => {
                Ok(ReadFileContentsResult::NotFound)
            }
            Err(source) => Err(FilesystemError::ReadMetadata {
                path: full_path,
                source,
            }),
        }
    }

    async fn set_file_contents(
        &self,
        relative_path: &Path,
        contents: &str,
    ) -> Result<(), FilesystemError> {
        let full_path = self.base_directory.join(relative_path);

        create_parent_directories(&full_path).await?;

        write(&full_path, contents)
            .await
            .map_err(|source| FilesystemError::WriteFile {
                path: full_path,
                source,
            })
    }
}
