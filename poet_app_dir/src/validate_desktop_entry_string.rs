use crate::app_dir_error::AppDirError;

pub fn validate_desktop_entry_string(value: &str) -> Result<String, AppDirError> {
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(AppDirError::InvalidDesktopEntryString {
            value: value.to_owned(),
        })
    } else {
        Ok(value.to_owned())
    }
}
