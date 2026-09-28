use std::io;

use poet::poet_error::PoetError;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetTestsError {
    #[error("unable to build the fixture project")]
    BuildFixtureProject(#[source] Box<PoetContentTestsError>),
    #[error("poet operation failed")]
    Poet(#[source] Box<PoetError>),
    #[error("unable to prepare test fixture on disk")]
    PrepareFixture(#[from] io::Error),
}

impl From<PoetContentTestsError> for PoetTestsError {
    fn from(poet_content_tests_error: PoetContentTestsError) -> Self {
        Self::BuildFixtureProject(Box::new(poet_content_tests_error))
    }
}

impl From<PoetError> for PoetTestsError {
    fn from(poet_error: PoetError) -> Self {
        Self::Poet(Box::new(poet_error))
    }
}
