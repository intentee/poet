use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn context_exposes_front_matter_reference_and_watch_mode() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_content_script::<_, String>(
            "context",
            fixture_docs_component_context("docs/second")?,
            r#"context.front_matter.title + "|" + context.reference.basename + "|" + context.reference.front_matter.description + "|" + context.front_matter.render + "|" + context.front_matter.props.len() + "|" + context.is_watching"#,
        )?,
        "Second|docs/second|d|true|0|false"
    );

    Ok(())
}
