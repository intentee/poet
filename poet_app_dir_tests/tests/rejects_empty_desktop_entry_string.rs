use poet_app_dir::app_dir_error::AppDirError;
use poet_app_dir::validate_desktop_entry_string::validate_desktop_entry_string;

#[test]
fn rejects_empty_desktop_entry_string() {
    assert!(matches!(
        validate_desktop_entry_string(""),
        Err(AppDirError::InvalidDesktopEntryString { value }) if value.is_empty()
    ));
}
