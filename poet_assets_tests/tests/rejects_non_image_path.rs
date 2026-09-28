use poet_assets::is_image_path::is_image_path;

#[test]
fn rejects_non_image_path() {
    assert!(!is_image_path("chunk-Q4UAENVW.js"));
}
