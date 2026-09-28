use poet_content::content_error::ContentError;
use poet_content_tests::content_error_of::content_error_of;
use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use rhai::Dynamic;

#[test]
fn context_rejects_primary_collection_of_document_without_collections()
-> Result<(), PoetContentTestsError> {
    assert!(
        evaluate_content_script::<_, Dynamic>(
            "context",
            fixture_docs_component_context("about")?,
            r"context.primary_collection",
        )
        .is_err_and(|eval_alt_result| matches!(
            content_error_of(&eval_alt_result),
            Some(ContentError::DocumentHasNoCollection { .. })
        ))
    );

    Ok(())
}
