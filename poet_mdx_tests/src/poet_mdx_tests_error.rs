use std::io;

use poet_filesystem::filesystem_error::FilesystemError;
use poet_mdx::mdx_error::MdxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetMdxTestsError {
    #[error("filesystem operation failed")]
    Filesystem(#[from] FilesystemError),
    #[error("MDX operation failed")]
    Mdx(#[from] MdxError),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
}
