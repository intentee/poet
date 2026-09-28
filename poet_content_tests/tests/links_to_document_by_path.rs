use poet_content::author_collection::AuthorCollection;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn links_to_document_by_path() -> Result<(), PoetContentTestsError> {
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
        content_document_linker.link_to("docs/guide"),
        Ok(link) if link == "/docs/guide/"
    ));

    Ok(())
}
