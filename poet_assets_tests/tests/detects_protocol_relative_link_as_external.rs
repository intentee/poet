use poet_assets::is_external_link::is_external_link;

#[test]
fn detects_protocol_relative_link_as_external() {
    assert!(is_external_link("//cdn.example.com/foo.js"));
}
