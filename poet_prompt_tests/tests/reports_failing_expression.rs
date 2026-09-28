use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn reports_failing_expression() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        evaluate_prompt_markdown("{missing_variable}")?,
        Err(PromptError::EvaluateExpression { expression, .. }) if expression == "missing_variable"
    ));

    Ok(())
}
