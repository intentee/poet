use std::fs::remove_dir_all;
use std::fs::write;

use poet_app_dir::app_dir_error::AppDirError;
use poet_app_dir::build_app_dir::build_app_dir;
use poet_app_dir::build_app_dir_params::BuildAppDirParams;
use poet_app_dir_tests::fixture_desktop_entry::fixture_desktop_entry;
use poet_app_dir_tests::fixture_site::FixtureSite;
use poet_app_dir_tests::poet_app_dir_tests_error::PoetAppDirTestsError;

#[tokio::test]
async fn rejects_unreadable_project_directory() -> Result<(), PoetAppDirTestsError> {
    let fixture_site = FixtureSite::create()?;
    let content_directory = fixture_site.source_directory.path().join("content");

    remove_dir_all(&content_directory)?;
    write(&content_directory, "not a directory")?;

    assert!(matches!(
        build_app_dir(BuildAppDirParams {
            desktop_entry: fixture_desktop_entry()?,
            output_directory: fixture_site.output_directory.path(),
            source_directory: fixture_site.source_directory.path(),
        })
        .await,
        Err(AppDirError::CopyProjectFiles(_))
    ));

    Ok(())
}
