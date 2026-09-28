use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_heading_with_generated_id() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"## Hello World", &SyntaxSet::new())??,
        r#"<h2 id="hello-world">Hello World</h2>"#
    );

    Ok(())
}
