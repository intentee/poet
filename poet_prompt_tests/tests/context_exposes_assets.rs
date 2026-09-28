use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn context_exposes_assets() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        assemble_prompt_messages(r#"**user**: {context.assets.image("logo.png")}"#)??,
        vec![PromptMessage {
            content: "/static/logo_ABCDEF12.png".into(),
            role: Role::User,
        },]
    );

    Ok(())
}
