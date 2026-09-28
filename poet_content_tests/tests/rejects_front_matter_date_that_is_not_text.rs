use poet_content::content_document_front_matter::ContentDocumentFrontMatter;

#[test]
fn rejects_front_matter_date_that_is_not_text() {
    assert!(
        toml::from_str::<ContentDocumentFrontMatter>(
            "description = \"d\"\nlast_updated_at = 5\nlayout = \"L\"\ntitle = \"t\"",
        )
        .is_err()
    );
}
