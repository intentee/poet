use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn context_uses_declared_primary_collection() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_content_script::<_, String>(
            "context",
            fixture_docs_component_context("multi/declared")?,
            r"context.primary_collection.name",
        )?,
        "guides"
    );

    Ok(())
}
