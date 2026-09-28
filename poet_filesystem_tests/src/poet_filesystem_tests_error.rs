use std::io;

use poet_filesystem::filesystem_error::FilesystemError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetFilesystemTestsError {
    #[error("filesystem operation failed")]
    Filesystem(#[from] FilesystemError),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
}
