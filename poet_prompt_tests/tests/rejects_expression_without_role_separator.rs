use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn rejects_expression_without_role_separator() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        assemble_prompt_messages(r#"{"x"} more"#)?,
        Err(PromptError::ContentWithoutRoleMarker { content }) if content == "x more"
    ));

    Ok(())
}
