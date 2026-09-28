use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn lengthens_inline_code_fence_around_backticks() -> Result<(), PoetPromptTestsError> {
    assert_eq!(evaluate_prompt_markdown("`` `a ``")??, "\n`` `a ``\n");

    Ok(())
}
