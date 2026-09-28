use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use poet_prompt_tests::assemble_prompt_messages::assemble_prompt_messages;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;

#[test]
fn context_exposes_front_matter() -> Result<(), PoetPromptTestsError> {
    assert_eq!(
        assemble_prompt_messages(
            "**user**: {context.front_matter.title} / {context.front_matter.description}"
        )??,
        vec![PromptMessage {
            content: "Fixture title / Fixture description".into(),
            role: Role::User,
        },]
    );

    Ok(())
}
