use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn assembles_messages_by_role() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        assemble_prompt_messages(
            "**user**: What is the capital of France?\n\n**assistant**: Paris."
        )??,
        vec![
            PromptMessage {
                content: "What is the capital of France?".into(),
                role: Role::User,
            },
            PromptMessage {
                content: "Paris.".into(),
                role: Role::Assistant,
            },
        ]
    );

    Ok(())
}
