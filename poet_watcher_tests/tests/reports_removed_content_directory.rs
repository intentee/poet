use notify_debouncer_full::notify::event::RemoveKind;
use std::collections::BTreeSet;

use notify_debouncer_full::notify::EventKind;
use poet_watcher::project_file_kind::ProjectFileKind;
use poet_watcher_tests::debounced_event::debounced_event;
use poet_watcher_tests::fixture_project::FixtureProject;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;

#[test]
fn reports_removed_content_directory() -> Result<(), PoetWatcherTestsError> {
    let fixture_project = FixtureProject::create()?;

    assert_eq!(
        fixture_project
            .project_file_classifier
            .changed_kinds(&[debounced_event(
                EventKind::Remove(RemoveKind::Folder),
                &fixture_project.directory.path().join("content/docs")
            ),]),
        BTreeSet::from([ProjectFileKind::Content])
    );

    Ok(())
}
