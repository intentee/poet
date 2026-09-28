use std::fs::remove_dir_all;

use poet::poet_error::PoetError;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn watch_reports_unwatchable_project() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;

    remove_dir_all(fixture_site.path().join("content"))?;
    fixture_site.write("content", "not a directory")?;

    assert!(matches!(
        run_poet_command(&["watch", &fixture_site.path_string()]).await?,
        Err(PoetError::WatchProjectFiles(_))
    ));

    Ok(())
}
