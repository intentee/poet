use poet_content::author_collection::AuthorCollection;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn passes_external_links_through() -> Result<(), PoetContentTestsError> {
    let content_document_linker = fixture_site_context(
        &[
            fixture_reference(
                "docs/guide",
                "description = \"d\"\nid = \"guide-id\"\nlayout = \"L\"\ntitle = \"Guide\"",
            )?,
            fixture_reference(
                "hidden",
                "description = \"d\"\nlayout = \"L\"\nrender = false\ntitle = \"Hidden\"",
            )?,
        ],
        AuthorCollection::default(),
    )?
    .content_document_linker;

    assert!(matches!(
        content_document_linker.resolve_link("https://example.com/docs"),
        Ok(link) if link == "https://example.com/docs"
    ));

    Ok(())
}
