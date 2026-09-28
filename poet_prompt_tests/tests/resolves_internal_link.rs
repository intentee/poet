use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn resolves_internal_link() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown("[guide](guide)")??,
        "\n[guide](/guide/)\n"
    );

    Ok(())
}
