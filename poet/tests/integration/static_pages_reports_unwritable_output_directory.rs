use std::fs::Permissions;
use std::fs::set_permissions;
use std::os::unix::fs::PermissionsExt as _;

use poet::poet_error::PoetError;
use tempfile::tempdir;

use crate::fixture_site::FixtureSite;
use crate::poet_tests_error::PoetTestsError;
use crate::run_poet_command::run_poet_command;

#[actix_web::test]
async fn static_pages_reports_unwritable_output_directory() -> Result<(), PoetTestsError> {
    let fixture_site = FixtureSite::create()?;
    let output_directory = tempdir()?;
    let output_path = output_directory.path().display().to_string();

    set_permissions(output_directory.path(), Permissions::from_mode(0o555))?;

    assert!(matches!(
        run_poet_command(&[
            "make",
            "static-pages",
            "--output-directory",
            &output_path,
            "--public-path",
            "/",
            &fixture_site.path_string()
        ])
        .await?,
        Err(PoetError::WriteGeneratedFiles(_))
    ));

    Ok(())
}
