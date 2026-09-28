use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn rejects_block_outside_of_message() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        assemble_prompt_messages("**user**: hi\n\n***")?,
        Err(PromptError::ContentWithoutRoleMarker { content }) if content == "---"
    ));

    Ok(())
}
