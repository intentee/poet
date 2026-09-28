use poet_content_tests::evaluate_content_script::evaluate_content_script;
use poet_content_tests::fixture_docs_component_context::fixture_docs_component_context;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;

#[test]
fn context_exposes_ranked_collection() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_content_script::<_, String>(
            "context",
            fixture_docs_component_context("docs/second")?,
            r#"let docs = context.collection("docs"); docs.name + ":" + docs.hierarchy.flat.len()"#,
        )?,
        "docs:5"
    );

    Ok(())
}
