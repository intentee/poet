use std::fs::create_dir_all;

use poet_app_dir::app_dir_error::AppDirError;
use poet_app_dir::build_app_dir::build_app_dir;
use poet_app_dir::build_app_dir_params::BuildAppDirParams;
use poet_app_dir_tests::fixture_desktop_entry::fixture_desktop_entry;
use poet_app_dir_tests::fixture_site::FixtureSite;
use poet_app_dir_tests::poet_app_dir_tests_error::PoetAppDirTestsError;

#[tokio::test]
async fn rejects_unwritable_app_run_file() -> Result<(), PoetAppDirTestsError> {
    let fixture_site = FixtureSite::create()?;
    let app_run_path = fixture_site
        .output_directory
        .path()
        .join("MySite.AppDir/AppRun");

    create_dir_all(&app_run_path)?;

    assert!(matches!(
        build_app_dir(BuildAppDirParams {
            desktop_entry: fixture_desktop_entry()?,
            output_directory: fixture_site.output_directory.path(),
            source_directory: fixture_site.source_directory.path(),
        })
        .await,
        Err(AppDirError::WriteAppDirFile { path, .. }) if path == app_run_path
    ));

    Ok(())
}
