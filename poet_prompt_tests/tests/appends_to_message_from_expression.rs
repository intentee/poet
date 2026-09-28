use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn appends_to_message_from_expression() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        assemble_prompt_messages(
            r#"**assistant**:

{context.append_to_message("wow")}"#
        )??,
        vec![PromptMessage {
            content: "wow".into(),
            role: Role::Assistant,
        },]
    );

    Ok(())
}
