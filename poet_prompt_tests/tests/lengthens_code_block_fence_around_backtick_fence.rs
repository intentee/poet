use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn lengthens_code_block_fence_around_backtick_fence() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown("````\n```\n````")??,
        "````\n```\n````"
    );

    Ok(())
}
