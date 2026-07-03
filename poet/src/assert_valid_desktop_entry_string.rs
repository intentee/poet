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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_string_when_valid() -> Result<()> {
        assert_eq!(
            assert_valid_desktop_entry_string("My Site")?,
            "My Site".to_string()
        );

        Ok(())
    }

    #[test]
    fn errors_when_string_contains_control_character() {
        assert!(assert_valid_desktop_entry_string("line\nbreak").is_err());
    }
}
