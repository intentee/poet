use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn renders_heading_with_space_after_marker() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown("### Section Title")??,
        "### Section Title"
    );

    Ok(())
}
