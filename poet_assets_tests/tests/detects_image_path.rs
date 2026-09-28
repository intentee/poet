use poet_assets::is_image_path::is_image_path;

#[test]
fn detects_image_path() {
    assert!(is_image_path("favicon_MWST2DE3.svg"));
}
