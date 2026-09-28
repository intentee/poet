use std::pin::pin;

use notify_debouncer_full::DebounceEventHandler as _;
use notify_debouncer_full::notify::EventKind;
use notify_debouncer_full::notify::event::ModifyKind;
use poet_watcher::project_file_change_notifier::ProjectFileChangeNotifier;
use poet_watcher::project_file_notifications::ProjectFileNotifications;
use poet_watcher_tests::debounced_event::debounced_event;
use poet_watcher_tests::fixture_project::FixtureProject;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;

#[tokio::test]
async fn notifies_about_every_changed_file_kind() -> Result<(), PoetWatcherTestsError> {
    let FixtureProject {
        directory,
        project_file_classifier,
    } = FixtureProject::create()?;
    let notifications = ProjectFileNotifications::default();
    let mut author_file_changed = pin!(notifications.on_author_file_changed.notified());
    let mut content_file_changed = pin!(notifications.on_content_file_changed.notified());
    let mut esbuild_metafile_changed = pin!(notifications.on_esbuild_metafile_changed.notified());
    let mut prompt_file_changed = pin!(notifications.on_prompt_file_changed.notified());
    let mut shortcode_file_changed = pin!(notifications.on_shortcode_file_changed.notified());

    author_file_changed.as_mut().enable();
    content_file_changed.as_mut().enable();
    esbuild_metafile_changed.as_mut().enable();
    prompt_file_changed.as_mut().enable();
    shortcode_file_changed.as_mut().enable();

    ProjectFileChangeNotifier {
        notifications: notifications.clone(),
        project_file_classifier,
    }
    .handle_event(Ok([
        "authors/ada.toml",
        "content/guide.md",
        "esbuild-meta.json",
        "prompts/greet.md",
        "shortcodes/Layout.rhai",
    ]
    .iter()
    .map(|relative_path| {
        debounced_event(
            EventKind::Modify(ModifyKind::Any),
            &directory.path().join(relative_path),
        )
    })
    .collect()));

    author_file_changed.await;
    content_file_changed.await;
    esbuild_metafile_changed.await;
    prompt_file_changed.await;
    shortcode_file_changed.await;

    Ok(())
}
