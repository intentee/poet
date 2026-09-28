use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn renders_evaluated_expression() -> Result<(), PoetContentTestsError> {
    assert_eq!(
        evaluate_markdown(r"Value {40 + 2}", &SyntaxSet::new())??,
        r"<p>Value 42</p>"
    );

    Ok(())
}
