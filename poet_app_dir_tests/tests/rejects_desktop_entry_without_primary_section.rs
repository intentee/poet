use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_error::AppDirError;

#[test]
fn rejects_desktop_entry_without_primary_section() {
    assert!(matches!(
        AppDirDesktopEntry::parse("[Other Section]\nName=mysite\n"),
        Err(AppDirError::DesktopEntrySectionMissing)
    ));
}
