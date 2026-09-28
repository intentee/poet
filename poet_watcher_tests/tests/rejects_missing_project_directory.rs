use poet_watcher::watch_project_files::watch_project_files;
use poet_watcher::watcher_error::WatcherError;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;
use tempfile::tempdir;

#[test]
fn rejects_missing_project_directory() -> Result<(), PoetWatcherTestsError> {
    let parent_directory = tempdir()?;
    let missing_directory = parent_directory.path().join("missing");

    assert!(matches!(
        watch_project_files(&missing_directory),
        Err(WatcherError::CanonicalizeSourceDirectory { path, .. }) if path == missing_directory
    ));

    Ok(())
}
