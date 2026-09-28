use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn rejects_unknown_role_marker() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        assemble_prompt_messages("**moderator**: hello")?,
        Err(PromptError::UnknownRole { role_name, .. }) if role_name == "moderator"
    ));

    Ok(())
}
