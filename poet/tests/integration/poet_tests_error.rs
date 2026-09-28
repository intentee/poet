use std::io;

use poet::poet_error::PoetError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetTestsError {
    #[error("unable to parse command line arguments")]
    ParseArguments(#[from] clap::Error),
    #[error("poet operation failed")]
    Poet(#[from] Box<PoetError>),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
}
