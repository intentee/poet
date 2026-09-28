use std::io;
use std::path::PathBuf;
use std::path::StripPrefixError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilesystemError {
    #[error("unable to create parent directories of '{path}'")]
    CreateParentDirectories {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("'{relative_path}' is not inside '{directory}'")]
    FileOutsideDirectory {
        directory: PathBuf,
        relative_path: PathBuf,
        #[source]
        source: StripPrefixError,
    },
    #[error("'{path}' is a directory, not a file")]
    FileIsDirectory { path: PathBuf },
    #[error("file '{path}' does not exist")]
    FileNotFound { path: PathBuf },
    #[error("path '{path}' is not valid UTF-8")]
    NonUtf8Path { path: PathBuf },
    #[error("unable to read directory '{path}'")]
    ReadDirectory {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to read an entry of directory '{path}'")]
    ReadDirectoryEntry {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to read file '{path}'")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to read metadata of '{path}'")]
    ReadMetadata {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("unable to write file '{path}'")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
