use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn context_checks_collection_membership() -> Result<(), PoetContentTestsError> {
    assert!(evaluate_content_script::<_, bool>(
        "context",
        fixture_docs_component_context("docs/second")?,
        r#"context.belongs_to("docs") && !context.belongs_to("guides")"#,
    )?);

    Ok(())
}
