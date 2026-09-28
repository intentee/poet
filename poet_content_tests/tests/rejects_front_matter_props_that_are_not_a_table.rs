use poet_content::content_document_front_matter::ContentDocumentFrontMatter;

#[test]
fn rejects_front_matter_props_that_are_not_a_table() {
    assert!(
        toml::from_str::<ContentDocumentFrontMatter>(
            "description = \"d\"\nlayout = \"L\"\nprops = 5\ntitle = \"t\"",
        )
        .is_err()
    );
}
