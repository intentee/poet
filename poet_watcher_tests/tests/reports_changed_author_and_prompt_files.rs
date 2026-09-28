use notify_debouncer_full::notify::event::ModifyKind;
use std::collections::BTreeSet;

use notify_debouncer_full::notify::EventKind;
use poet_watcher::project_file_kind::ProjectFileKind;
use poet_watcher_tests::debounced_event::debounced_event;
use poet_watcher_tests::fixture_project::FixtureProject;
use poet_watcher_tests::poet_watcher_tests_error::PoetWatcherTestsError;

#[test]
fn reports_changed_author_and_prompt_files() -> Result<(), PoetWatcherTestsError> {
    let fixture_project = FixtureProject::create()?;

    assert_eq!(
        fixture_project.project_file_classifier.changed_kinds(&[
            debounced_event(
                EventKind::Modify(ModifyKind::Any),
                &fixture_project.directory.path().join("authors/ada.toml")
            ),
            debounced_event(
                EventKind::Modify(ModifyKind::Any),
                &fixture_project.directory.path().join("prompts/greet.md")
            ),
        ]),
        BTreeSet::from([ProjectFileKind::Author, ProjectFileKind::Prompt])
    );

    Ok(())
}
