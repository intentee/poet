use poet_content::author_collection::AuthorCollection;
use poet_content::content_error::ContentError;
use poet_content_tests::fixture_reference::fixture_reference;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use poet_mdx::document_error::DocumentError;

#[test]
fn rejects_missing_succeeding_document() -> Result<(), PoetContentTestsError> {
    let Err(ContentError::InvalidDocuments(document_errors)) = fixture_site_context(
        &[fixture_reference(
            "first",
            "description = \"d\"\nlayout = \"L\"\ntitle = \"t\"\n\n[[collection]]\nafter = \"ghost\"\nname = \"docs\"",
        )?],
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
                    error: ContentError::SucceedingDocumentNotFound { .. },
                    ..
                } if true
            ))
    );

    Ok(())
}
