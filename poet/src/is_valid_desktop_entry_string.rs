// Values of type string may contain all ASCII characters except for control characters.
pub fn is_valid_desktop_entry_string(input: &str) -> bool {
    !input.is_empty() && !input.chars().any(|char| char.is_control())
}
