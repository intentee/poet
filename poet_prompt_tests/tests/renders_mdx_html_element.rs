use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn renders_mdx_html_element() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown("<div>\ntext\n</div>")??,
        "<div >\ntext\n</div>"
    );

    Ok(())
}
