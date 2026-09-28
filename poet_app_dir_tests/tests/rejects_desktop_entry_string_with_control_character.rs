use poet_app_dir::app_dir_error::AppDirError;
use poet_app_dir::validate_desktop_entry_string::validate_desktop_entry_string;

#[test]
fn rejects_desktop_entry_string_with_control_character() {
    assert!(matches!(
        validate_desktop_entry_string("line\nbreak"),
        Err(AppDirError::InvalidDesktopEntryString { value }) if value == "line\nbreak"
    ));
}
