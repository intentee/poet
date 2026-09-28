use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn context_links_to_documents() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_content_script::<_, String>(
            "context",
            fixture_docs_component_context("docs/second")?,
            r##"context.link_to("#docs-home") + " " + context.reference.canonical_link + " " + context.reference.basename_last_stem"##,
        )?,
        "/docs/ /docs/second/ second"
    );

    Ok(())
}
