use poet_app_dir_tests::fixture_desktop_entry::fixture_desktop_entry;
use poet_app_dir_tests::poet_app_dir_tests_error::PoetAppDirTestsError;

#[test]
fn renders_desktop_entry() -> Result<(), PoetAppDirTestsError> {
    assert_eq!(
        fixture_desktop_entry()?.to_string(),
        "[Desktop Entry]\nCategories=System;\nIcon=MySite\nName=MySite\nTerminal=true\nType=Application\nX-ImplementationTitle=My Site\nX-PoetVersion=0.8.0\nX-SiteVersion=1.2.3\n"
    );

    Ok(())
}
