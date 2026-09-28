use poet_prompt::prompt_error::PromptError;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn rejects_message_without_role() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        assemble_prompt_messages(": orphan")?,
        Err(PromptError::MessageWithoutRole { content }) if content == "orphan"
    ));

    Ok(())
}
