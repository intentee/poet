use tempfile::tempdir;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn app_dir_builds_app_dir() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;
    let output_directory = tempdir()?;

    assert!(matches!(
        run_poet_command(&[
            "make",
            "app-dir",
            "--name",
            "fixture",
            "--output-directory",
            &output_directory.path().display().to_string(),
            "--title",
            "Fixture",
            "--version",
            "1.0.0",
            &fixture_site.path_string(),
        ])
        .await?,
        Ok(())
    ));

    assert!(
        output_directory
            .path()
            .join("fixture.AppDir/fixture.desktop")
            .is_file()
    );

    Ok(())
}
