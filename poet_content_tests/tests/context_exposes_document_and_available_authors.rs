use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn context_exposes_document_and_available_authors() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_content_script::<_, String>(
            "context",
            fixture_docs_component_context("docs/second")?,
            r#"context.authors[0].basename + ":" + context.authors[0].data.name + ":" + context.available_authors.len()"#,
        )?,
        "alice:Alice:2"
    );

    Ok(())
}
