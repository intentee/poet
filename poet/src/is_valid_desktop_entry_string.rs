// Values of type string may contain all ASCII characters except for control characters.
pub fn is_valid_desktop_entry_string(input: &str) -> bool {
    !input.is_empty() && !input.chars().any(|char| char.is_control())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_printable_string() {
        assert!(is_valid_desktop_entry_string("My Site"));
    }

    #[test]
    fn rejects_empty_string() {
        assert!(!is_valid_desktop_entry_string(""));
    }

    #[test]
    fn rejects_string_with_control_character() {
        assert!(!is_valid_desktop_entry_string("line\nbreak"));
    }
}
