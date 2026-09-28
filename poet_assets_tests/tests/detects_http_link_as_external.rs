use poet_assets::is_external_link::is_external_link;

#[test]
fn detects_http_link_as_external() {
    assert!(is_external_link("http://example.com/style.css"));
}
