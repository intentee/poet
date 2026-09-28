use std::fmt;

use crate::app_dir_error::AppDirError;
use crate::validate_desktop_entry_string::validate_desktop_entry_string;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppDirName {
    name: String,
}

impl AppDirName {
    pub fn parse(name: &str) -> Result<Self, AppDirError> {
        validate_desktop_entry_string(name).map(|name| Self { name })
    }

    #[must_use]
    pub fn app_dir_directory_name(&self) -> String {
        format!("{}.AppDir", self.name)
    }

    #[must_use]
    pub fn desktop_file_name(&self) -> String {
        format!("{}.desktop", self.name.to_lowercase())
    }

    #[must_use]
    pub fn icon_file_name(&self) -> String {
        format!("{}.svg", self.name.to_lowercase())
    }
}

impl fmt::Display for AppDirName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}
