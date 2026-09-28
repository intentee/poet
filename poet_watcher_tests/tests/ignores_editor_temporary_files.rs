use notify_debouncer_full::notify::event::ModifyKind;
use std::collections::BTreeSet;

use notify_debouncer_full::notify::EventKind;
use poet_watcher_tests::debounced_event::debounced_event;
use poet_watcher_tests::fixture_project::FixtureProject;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;

#[test]
fn ignores_editor_temporary_files() -> Result<(), PoetWatcherTestsError> {
    let fixture_project = FixtureProject::create()?;

    assert_eq!(
        fixture_project.project_file_classifier.changed_kinds(&[
            debounced_event(
                EventKind::Modify(ModifyKind::Any),
                &fixture_project.directory.path().join("content/guide.md~")
            ),
            debounced_event(
                EventKind::Modify(ModifyKind::Any),
                &fixture_project
                    .directory
                    .path()
                    .join("content/.guide.md.swp")
            ),
        ]),
        BTreeSet::new()
    );

    Ok(())
}
