use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_error::AppDirError;

#[test]
fn rejects_malformed_desktop_entry() {
    assert!(matches!(
        AppDirDesktopEntry::parse("[Desktop Entry\nName=mysite\n"),
        Err(AppDirError::ParseDesktopEntry(_))
    ));
}
