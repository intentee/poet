use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_search::search_error::SearchError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoetSearchTestsError {
    #[error("unable to build the fixture project")]
    BuildFixtureProject(#[from] PoetContentTestsError),
    #[error("search operation failed")]
    Search(#[from] SearchError),
}
