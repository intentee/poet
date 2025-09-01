pub fn is_external_link(link: &str) -> bool {
    link.starts_with("http:") || link.starts_with("https://")
}
