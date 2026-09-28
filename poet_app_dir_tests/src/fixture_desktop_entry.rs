use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_error::AppDirError;
use poet_app_dir::app_dir_name::AppDirName;

pub fn fixture_desktop_entry() -> Result<AppDirDesktopEntry, AppDirError> {
    Ok(AppDirDesktopEntry {
        name: AppDirName::parse("MySite")?,
        poet_version: "0.8.0".to_owned(),
        site_version: "1.2.3".to_owned(),
        title: "My Site".to_owned(),
    })
}
