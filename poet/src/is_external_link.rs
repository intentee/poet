pub fn is_external_link(link: &str) -> bool {
    link.starts_with("http:")
        || link.starts_with("https:")
        || link.starts_with("//")
        || link.starts_with("mailto:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_https_as_external() {
        assert!(is_external_link("https://example.com/script.js"));
    }

    #[test]
    fn detects_http_as_external() {
        assert!(is_external_link("http://example.com/style.css"));
    }

    #[test]
    fn detects_protocol_relative_as_external() {
        assert!(is_external_link("//cdn.example.com/foo.js"));
    }

    #[test]
    fn detects_mailto_as_external() {
        assert!(is_external_link("mailto:user@example.com"));
    }

    #[test]
    fn treats_relative_path_as_internal() {
        assert!(!is_external_link("assets/main.js"));
    }

    #[test]
    fn treats_absolute_path_as_internal() {
        assert!(!is_external_link("/assets/main.js"));
    }
}
