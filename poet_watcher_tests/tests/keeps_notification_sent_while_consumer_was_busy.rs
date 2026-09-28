use futures_util::FutureExt as _;
use notify_debouncer_full::DebounceEventHandler as _;
use notify_debouncer_full::notify::EventKind;
use notify_debouncer_full::notify::event::ModifyKind;
use poet_watcher::project_file_change_notifier::ProjectFileChangeNotifier;
use poet_watcher::project_file_notifications::ProjectFileNotifications;
use poet_watcher_tests::debounced_event::debounced_event;
use poet_watcher_tests::fixture_project::FixtureProject;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;

#[test]
fn keeps_notification_sent_while_consumer_was_busy() -> Result<(), PoetWatcherTestsError> {
    let FixtureProject {
        directory,
        project_file_classifier,
    } = FixtureProject::create()?;
    let notifications = ProjectFileNotifications::default();

    ProjectFileChangeNotifier {
        notifications: notifications.clone(),
        project_file_classifier,
    }
    .handle_event(Ok(vec![debounced_event(
        EventKind::Modify(ModifyKind::Any),
        &directory.path().join("content/guide.md"),
    )]));

    assert!(
        notifications
            .on_content_file_changed
            .notified()
            .now_or_never()
            .is_some()
    );

    Ok(())
}
