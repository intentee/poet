use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn renders_table_cells() -> Result<(), PoetPromptTestsError> {
    let rendered = evaluate_prompt_markdown("| a | b |\n| - | - |\n| 1 | 2 |")??;

    assert!(rendered.contains("| a"));
    assert!(rendered.contains("| 1"));

    Ok(())
}
