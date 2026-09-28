use std::io;

use poet_app_dir::app_dir_error::AppDirError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetAppDirTestsError {
    #[error("AppDir operation failed")]
    AppDir(#[from] AppDirError),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
}
