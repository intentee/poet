use std::fs::write;

use poet_watcher::watch_project_files::watch_project_files;
use poet_watcher::watcher_error::WatcherError;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;
use tempfile::tempdir;

#[test]
fn rejects_file_in_place_of_watched_directory() -> Result<(), PoetWatcherTestsError> {
    let project_directory = tempdir()?;
    let content_path = project_directory.path().canonicalize()?.join("content");

    write(&content_path, "not a directory")?;

    assert!(matches!(
        watch_project_files(project_directory.path()),
        Err(WatcherError::CreateWatchedDirectory { path, .. }) if path == content_path
    ));

    Ok(())
}
