use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn renders_inline_code_verbatim() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown(r#"`say "hi"`"#)??,
        r#"
`say "hi"`
"#
    );

    Ok(())
}
