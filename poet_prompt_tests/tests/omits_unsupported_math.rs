use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn omits_unsupported_math() -> Result<(), PoetPromptTestsError> {
    assert_eq!(evaluate_prompt_markdown("$$\nx\n$$")??, "");

    Ok(())
}
