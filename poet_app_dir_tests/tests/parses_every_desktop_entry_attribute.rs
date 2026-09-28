use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir_tests::poet_app_dir_tests_error::PoetAppDirTestsError;

#[test]
fn parses_every_desktop_entry_attribute() -> Result<(), PoetAppDirTestsError> {
    let AppDirDesktopEntry {
        name,
        poet_version,
        site_version,
        title,
    } = AppDirDesktopEntry::parse(
        "[Desktop Entry]\nName=mysite\nX-PoetVersion=0.6.2\nX-SiteVersion=1.2.3\nX-ImplementationTitle=My Site\n",
    )?;

    assert_eq!(name.to_string(), "mysite");
    assert_eq!(poet_version, "0.6.2");
    assert_eq!(site_version, "1.2.3");
    assert_eq!(title, "My Site");

    Ok(())
}
