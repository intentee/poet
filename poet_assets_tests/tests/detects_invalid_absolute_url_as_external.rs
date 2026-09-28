use poet_assets::is_external_link::is_external_link;

#[test]
fn detects_invalid_absolute_url_as_external() {
    assert!(is_external_link("http://[invalid"));
}
