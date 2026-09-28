use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_error::AppDirError;

#[test]
fn rejects_ambiguous_desktop_entry_attribute() {
    assert!(matches!(
        AppDirDesktopEntry::parse(
            "[Desktop Entry]\nName=mysite\nName=othersite\nX-PoetVersion=0.6.2\nX-SiteVersion=1.2.3\nX-ImplementationTitle=My Site\n"
        ),
        Err(AppDirError::AmbiguousDesktopEntryAttribute { key }) if key == "Name"
    ));
}
