use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn rejects_image_of_missing_asset() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        evaluate_prompt_markdown("![x](missing.png)")?,
        Err(PromptError::ResolveImage { url, .. }) if url == "missing.png"
    ));

    Ok(())
}
