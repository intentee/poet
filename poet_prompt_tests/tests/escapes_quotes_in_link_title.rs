use poet_prompt_tests::evaluate_prompt_markdown::evaluate_prompt_markdown;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn escapes_quotes_in_link_title() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        evaluate_prompt_markdown(r#"[site](https://example.com "Say \"hi\"")"#)??,
        r#"
[site](https://example.com "Say &quot;hi&quot;")
"#
    );

    Ok(())
}
