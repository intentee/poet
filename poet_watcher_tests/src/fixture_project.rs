use poet_watcher::project_file_classifier::ProjectFileClassifier;
use tempfile::TempDir;
use tempfile::tempdir;

use crate::poet_watcher_tests_error::PoetWatcherTestsError;

pub struct FixtureProject {
    pub directory: TempDir,
    pub project_file_classifier: ProjectFileClassifier,
}

impl FixtureProject {
    pub fn create() -> Result<Self, PoetWatcherTestsError> {
        let directory = tempdir()?;

        Ok(Self {
            project_file_classifier: ProjectFileClassifier::create_watched_directories_in(
                directory.path(),
            )?,
            directory,
        })
    }
}
