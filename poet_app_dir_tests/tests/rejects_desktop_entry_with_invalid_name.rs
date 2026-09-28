use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_error::AppDirError;

#[test]
fn rejects_desktop_entry_with_invalid_name() {
    assert!(matches!(
        AppDirDesktopEntry::parse(
            "[Desktop Entry]\nName=\nX-PoetVersion=0.6.2\nX-SiteVersion=1.2.3\nX-ImplementationTitle=My Site\n"
        ),
        Err(AppDirError::InvalidDesktopEntryString { value }) if value.is_empty()
    ));
}
