use poet_content::content_error::ContentError;
use poet_content_tests::evaluate_markdown::evaluate_markdown;
use poet_content_tests::poet_content_tests_error::PoetContentTestsError;
use syntect::parsing::SyntaxSet;

#[test]
fn rejects_invalid_code_block_metadata() -> Result<(), PoetContentTestsError> {
    assert!(matches!(
        evaluate_markdown(
            r#"```text bad:"unterminated
code
```"#,
            &SyntaxSet::new()
        )?,
        Err(ContentError::ParseCodeMetadata { .. })
    ));

    Ok(())
}
