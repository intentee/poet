use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn renders_list_items() -> Result<(), PoetPromptTestsError> {
    let rendered = evaluate_prompt_markdown("- first\n- second")??;

    assert!(rendered.contains("- "));
    assert!(rendered.contains("first"));
    assert!(rendered.contains("second"));

    Ok(())
}
