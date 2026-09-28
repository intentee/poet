use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn keeps_inline_code_surrounded_by_spaces() -> Result<(), PoetPromptTestsError> {
    assert_eq!(evaluate_prompt_markdown("`  a  `")??, "\n`  a  `\n");

    Ok(())
}
