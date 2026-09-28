use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::expression_system_error_of::expression_system_error_of;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn context_rejects_unknown_role() -> Result<(), PoetPromptTestsError> {
    let Err(prompt_error) = assemble_prompt_messages(r#"{context.switch_role_to("moderator")}"#)?
    else {
        panic!("expected the role switch to fail");
    };

    assert!(matches!(
        expression_system_error_of::<PromptError>(&prompt_error),
        Some(PromptError::UnknownRole { role_name, .. }) if role_name == "moderator"
    ));

    Ok(())
}
