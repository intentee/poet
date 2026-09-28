use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn appends_separated_paragraph_to_switched_role() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        assemble_prompt_messages(
            r#"{context.switch_role_to("assistant")}

: Paris."#
        )??,
        vec![PromptMessage {
            content: "Paris.".into(),
            role: Role::Assistant,
        },]
    );

    Ok(())
}
