use poet_app_dir::app_dir_name::AppDirName;
use poet_app_dir_tests::poet_app_dir_tests_error::PoetAppDirTestsError;

#[test]
fn names_app_dir_files_after_app_name() -> Result<(), PoetAppDirTestsError> {
    let app_dir_name = AppDirName::parse("MySite")?;

    assert_eq!(app_dir_name.app_dir_directory_name(), "MySite.AppDir");
    assert_eq!(app_dir_name.desktop_file_name(), "mysite.desktop");
    assert_eq!(app_dir_name.icon_file_name(), "mysite.svg");

    Ok(())
}
