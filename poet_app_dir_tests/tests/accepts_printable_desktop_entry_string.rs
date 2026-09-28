use poet_app_dir::validate_desktop_entry_string::validate_desktop_entry_string;
use poet_app_dir_tests::poet_app_dir_tests_error::PoetAppDirTestsError;

#[test]
fn accepts_printable_desktop_entry_string() -> Result<(), PoetAppDirTestsError> {
    assert_eq!(validate_desktop_entry_string("My Site")?, "My Site");

    Ok(())
}
