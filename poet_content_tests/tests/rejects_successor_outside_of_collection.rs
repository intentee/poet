use poet_content::author_collection::AuthorCollection;
use poet_content::content_error::ContentError;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn rejects_successor_outside_of_collection() -> Result<(), PoetContentTestsError> {
    let site_context = fixture_site_context(
        &[
            fixture_reference(
                "first",
                "description = \"d\"\nlayout = \"L\"\ntitle = \"t\"\n\n[[collection]]\nafter = \"second\"\nname = \"one\"",
            )?,
            fixture_reference(
                "second",
                "description = \"d\"\nlayout = \"L\"\ntitle = \"t\"\n\n[[collection]]\nname = \"two\"",
            )?,
        ],
        AuthorCollection::default(),
    );

    assert!(matches!(
        site_context,
        Err(ContentError::AfterDocumentNotInCollection { collection_name, .. }) if collection_name == "one"
    ));

    Ok(())
}
