use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn context_links_to_documents() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        assemble_prompt_messages(r#"**user**: {context.link_to("guide")}"#)??,
        vec![PromptMessage {
            content: "/guide/".into(),
            role: Role::User,
        },]
    );

    Ok(())
}
