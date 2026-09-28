use poet_app_dir::app_dir_desktop_entry::AppDirDesktopEntry;
use poet_app_dir::app_dir_error::AppDirError;

struct RequiredAttribute {
    key: &'static str,
    value: &'static str,
}

const REQUIRED_ATTRIBUTES: [RequiredAttribute; 4] = [
    RequiredAttribute {
        key: "Name",
        value: "mysite",
    },
    RequiredAttribute {
        key: "X-PoetVersion",
        value: "0.6.2",
    },
    RequiredAttribute {
        key: "X-SiteVersion",
        value: "1.2.3",
    },
    RequiredAttribute {
        key: "X-ImplementationTitle",
        value: "My Site",
    },
];

#[test]
fn rejects_desktop_entry_without_any_required_attribute() {
    for missing_attribute in &REQUIRED_ATTRIBUTES {
        let desktop_entry = REQUIRED_ATTRIBUTES
            .iter()
            .filter(|attribute| attribute.key != missing_attribute.key)
            .fold(
                "[Desktop Entry]\n".to_owned(),
                |desktop_entry, RequiredAttribute { key, value }| {
                    format!("{desktop_entry}{key}={value}\n")
                },
            );

        assert!(matches!(
            AppDirDesktopEntry::parse(&desktop_entry),
            Err(AppDirError::DesktopEntryAttributeMissing { key }) if key == missing_attribute.key
        ));
    }
}
