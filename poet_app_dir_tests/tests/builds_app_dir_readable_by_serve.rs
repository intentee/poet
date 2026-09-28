use std::fs::metadata;
use std::fs::read_to_string;
use std::os::unix::fs::PermissionsExt as _;

use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_name::AppDirName;
use poet_app_dir::build_app_dir::build_app_dir;
use poet_app_dir::build_app_dir_params::BuildAppDirParams;
use poet_app_dir_tests::fixture_desktop_entry::fixture_desktop_entry;
use poet_app_dir_tests::fixture_site::FixtureSite;
use poet_app_dir_tests::poet_app_dir_tests_error::PoetAppDirTestsError;
use poet_filesystem::storage::Storage;

#[tokio::test]
async fn builds_app_dir_readable_by_serve() -> Result<(), PoetAppDirTestsError> {
    let fixture_site = FixtureSite::create()?;
    let app_dir_path = build_app_dir(BuildAppDirParams {
        desktop_entry: fixture_desktop_entry()?,
        output_directory: fixture_site.output_directory.path(),
        source_directory: fixture_site.source_directory.path(),
    })
    .await?;
    let AppDirDesktopEntry {
        name, site_version, ..
    } = AppDirDesktopEntry::read_from(
        &Storage {
            base_directory: app_dir_path.clone(),
        },
        &AppDirName::parse("MySite")?,
    )
    .await?;

    assert_eq!(name.to_string(), "MySite");
    assert_eq!(site_version, "1.2.3");
    assert_eq!(
        metadata(app_dir_path.join("AppRun"))?.permissions().mode() & 0o777,
        0o755
    );
    assert!(read_to_string(app_dir_path.join("AppRun"))?.contains("--app-name \"MySite\""));
    assert!(app_dir_path.join("mysite.svg").is_file());
    assert!(app_dir_path.join("authors/ada.toml").is_file());
    assert!(app_dir_path.join("content/index.md").is_file());
    assert!(app_dir_path.join("prompts/greet.md").is_file());
    assert!(app_dir_path.join("shortcodes/Layout.rhai").is_file());
    assert!(app_dir_path.join("esbuild-meta.json").is_file());
    assert!(app_dir_path.join("static/logo_ABCDEF12.png").is_file());

    Ok(())
}
