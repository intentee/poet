#[must_use]
pub fn is_image_path(path: &str) -> bool {
    mime_guess::from_path(path).first_or_octet_stream().type_() == mime::IMAGE
}
