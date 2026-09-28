use poet_content::content_document_front_matter::ContentDocumentFrontMatter;

const FRONT_MATTER: &str = "description = \"d\"\nlayout = \"L\"\nprops = 5\ntitle = \"t\"";

#[test]
fn rejects_front_matter_props_that_are_not_a_table() {
    assert!(matches!(
        toml::from_str::<ContentDocumentFrontMatter>(FRONT_MATTER),
        Err(toml_error) if toml_error.span().is_some_and(|span| &FRONT_MATTER[span] == "5")
    ));
}
