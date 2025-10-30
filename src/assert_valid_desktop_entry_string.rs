use anyhow::Result;
use anyhow::anyhow;

use crate::is_valid_desktop_entry_string::is_valid_desktop_entry_string;

pub fn assert_valid_desktop_entry_string(input: &str) -> Result<String> {
    if is_valid_desktop_entry_string(input) {
        Ok(input.to_string())
    } else {
        Err(anyhow!(
            "Desktop entry strings cannot contain entry characters: '{input}'"
        ))
    }
}
