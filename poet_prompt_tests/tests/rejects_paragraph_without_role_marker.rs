use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn rejects_paragraph_without_role_marker() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        assemble_prompt_messages("Just text.")?,
        Err(PromptError::ContentWithoutRoleMarker { content }) if content == "Just text."
    ));

    Ok(())
}
