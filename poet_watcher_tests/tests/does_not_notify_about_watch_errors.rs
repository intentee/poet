use std::pin::pin;

use futures_util::FutureExt as _;
use notify_debouncer_full::DebounceEventHandler as _;
use notify_debouncer_full::notify::Error as NotifyError;
use poet_watcher::project_file_change_notifier::ProjectFileChangeNotifier;
use poet_watcher::project_file_notifications::ProjectFileNotifications;
use poet_watcher_tests::fixture_project::FixtureProject;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;

#[test]
fn does_not_notify_about_watch_errors() -> Result<(), PoetWatcherTestsError> {
    let FixtureProject {
        project_file_classifier,
        ..
    } = FixtureProject::create()?;
    let notifications = ProjectFileNotifications::default();
    let mut content_file_changed = pin!(notifications.on_content_file_changed.notified());

    content_file_changed.as_mut().enable();

    ProjectFileChangeNotifier {
        notifications: notifications.clone(),
        project_file_classifier,
    }
    .handle_event(Err(vec![NotifyError::generic("watch failed")]));

    assert!(content_file_changed.as_mut().now_or_never().is_none());

    Ok(())
}
