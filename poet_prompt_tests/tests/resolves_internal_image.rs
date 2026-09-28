use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn resolves_internal_image() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown("![logo](logo.png)")??,
        "\n![logo](/static/logo_ABCDEF12.png)\n"
    );

    Ok(())
}
