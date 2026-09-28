use poet_content::content_document_front_matter::ContentDocumentFrontMatter;

const FRONT_MATTER: &str =
    "description = \"d\"\nlast_updated_at = 5\nlayout = \"L\"\ntitle = \"t\"";

#[test]
fn rejects_front_matter_date_that_is_not_text() {
    assert!(matches!(
        toml::from_str::<ContentDocumentFrontMatter>(FRONT_MATTER),
        Err(toml_error) if toml_error.span().is_some_and(|span| &FRONT_MATTER[span] == "5")
    ));
}
