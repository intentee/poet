use poet::poet_error::PoetError;
use poet_app_dir::app_dir_error::AppDirError;
use tempfile::tempdir;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn app_dir_rejects_invalid_title() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;
    let output_directory = tempdir()?;
    let output_path = output_directory.path().display().to_string();

    assert!(matches!(
        run_poet_command(&["make", "app-dir", "--name", "fixture", "--output-directory", &output_path, "--title", "Fix\nture", "--version", "1.0.0", &fixture_site.path_string()]).await?,
        Err(PoetError::AppDir(AppDirError::InvalidDesktopEntryString { value })) if value == "Fix\nture"
    ));

    Ok(())
}
