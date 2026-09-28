use poet::poet_error::PoetError;
use poet_app_dir::app_dir_error::AppDirError;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn serve_rejects_invalid_app_name() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;

    assert!(matches!(
        run_poet_command(&[
            "serve",
            "--app-name",
            "",
            "--public-path",
            "/",
            &fixture_site.path_string()
        ])
        .await?,
        Err(PoetError::AppDir(
            AppDirError::InvalidDesktopEntryString { .. }
        ))
    ));

    Ok(())
}
