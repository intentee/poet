use std::fs::write;
use std::pin::pin;

use poet_watcher::project_file_watcher::ProjectFileWatcher;
use poet_watcher::watch_project_files::watch_project_files;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;
use tempfile::tempdir;

#[tokio::test]
async fn detects_created_content_file() -> Result<(), PoetWatcherTestsError> {
    let project_directory = tempdir()?;
    let ProjectFileWatcher {
        debouncer: _debouncer,
        notifications,
    } = watch_project_files(project_directory.path())?;
    let mut content_file_changed = pin!(notifications.on_content_file_changed.notified());

    content_file_changed.as_mut().enable();

    write(project_directory.path().join("content/guide.md"), "Guide")?;

    content_file_changed.await;

    Ok(())
}
