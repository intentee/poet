pub fn is_image_path(path: &str) -> bool {
    mime_guess::from_path(path).first_or_octet_stream().type_() == mime::IMAGE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_image_extension() {
        assert!(is_image_path("favicon_MWST2DE3.svg"));
    }

    #[test]
    fn rejects_non_image_extension() {
        assert!(!is_image_path("chunk-Q4UAENVW.js"));
    }
}
