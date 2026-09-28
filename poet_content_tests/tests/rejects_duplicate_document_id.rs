use poet_content::author_collection::AuthorCollection;
use poet_content::content_error::ContentError;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::document_error::DocumentError;

#[test]
fn rejects_duplicate_document_id() -> Result<(), PoetContentTestsError> {
    let Err(ContentError::InvalidDocuments(document_errors)) = fixture_site_context(
        &[
            fixture_reference(
                "first",
                "description = \"d\"\nid = \"same\"\nlayout = \"L\"\ntitle = \"t\"",
            )?,
            fixture_reference(
                "second",
                "description = \"d\"\nid = \"same\"\nlayout = \"L\"\ntitle = \"t\"",
            )?,
        ],
        AuthorCollection::default(),
    ) else {
        panic!("expected invalid documents");
    };

    assert!(
        document_errors
            .into_document_errors()
            .iter()
            .any(|document_error| matches!(
                document_error,
                DocumentError {
                    error: ContentError::DuplicateDocumentId { id, .. },
                    ..
                } if id == "same"
            ))
    );

    Ok(())
}
