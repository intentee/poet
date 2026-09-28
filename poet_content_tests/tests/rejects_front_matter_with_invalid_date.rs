use poet_content::content_document_front_matter::ContentDocumentFrontMatter;

#[test]
fn rejects_front_matter_with_invalid_date() {
    assert!(
        toml::from_str::<ContentDocumentFrontMatter>(
            "description = \"d\"\nlast_updated_at = \"yesterday\"\nlayout = \"L\"\ntitle = \"t\"",
        )
        .is_err()
    );
}
