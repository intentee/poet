use notify_debouncer_full::notify::event::ModifyKind;
use std::collections::BTreeSet;

use notify_debouncer_full::notify::EventKind;
use poet_watcher_tests::debounced_event::debounced_event;
use poet_watcher_tests::fixture_project::FixtureProject;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;

#[test]
fn ignores_files_outside_project_directories() -> Result<(), PoetWatcherTestsError> {
    let fixture_project = FixtureProject::create()?;

    assert_eq!(
        fixture_project
            .project_file_classifier
            .changed_kinds(&[debounced_event(
                EventKind::Modify(ModifyKind::Any),
                &fixture_project.directory.path().join("README.md")
            ),]),
        BTreeSet::new()
    );

    Ok(())
}
