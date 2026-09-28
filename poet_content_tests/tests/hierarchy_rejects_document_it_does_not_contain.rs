use poet_content::author_collection::AuthorCollection;
use poet_content::content_error::ContentError;
use poet_content_tests::content_error_of::content_error_of;
use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_references::fixture_docs_references;
use poet_content_tests::fixture_site_context::fixture_site_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use rhai::Dynamic;

#[test]
fn hierarchy_rejects_document_it_does_not_contain() -> Result<(), PoetContentTestsError> {
    let site_context =
        fixture_site_context(&fixture_docs_references()?, AuthorCollection::default())?;

    assert!(
        evaluate_content_script::<_, Dynamic>(
            "hierarchy",
            site_context.content_document_collections_ranked["docs"]
                .hierarchy
                .clone(),
            r#"hierarchy.after("about")"#,
        )
        .is_err_and(|eval_alt_result| matches!(
            content_error_of(&eval_alt_result),
            Some(ContentError::HierarchyDocumentNotFound { .. })
        ))
    );

    Ok(())
}
