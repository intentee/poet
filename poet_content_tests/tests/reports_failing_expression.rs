use poet_content::content_error::ContentError;
use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn reports_failing_expression() -> Result<(), PoetContentTestsError> {
    assert!(matches!(
        evaluate_markdown(r"{undefined_variable}", &SyntaxSet::new())?,
        Err(ContentError::EvaluateExpression { expression, .. }) if expression == "undefined_variable"
    ));

    Ok(())
}
