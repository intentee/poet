use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::prompts_get_result::PromptsGetResult;
use poet_mcp::role::Role;
use poet_prompt_tests::fixture_argument_input::FixtureArgumentInput;
use poet_prompt_tests::poet_prompt_tests_error::PoetPromptTestsError;
use poet_prompt_tests::respond_to_prompt_document::respond_to_prompt_document;

const DOCUMENT: &str = r#"+++
description = "Helps with finishing a task"
title = "Help me with finishing the task"

[arguments.objective]
description = "Describe what you are trying to do"
required = true
title = "Your objective"
+++

**user**: {context.arguments.objective.input} ({context.arguments.objective.title}, {context.arguments.objective.description}, {context.arguments.objective.required})
"#;

#[test]
fn exposes_arguments_to_expressions() -> Result<(), PoetPromptTestsError> {
    let PromptsGetResult {
        description,
        messages,
        ..
    } = respond_to_prompt_document(
        DOCUMENT,
        &[FixtureArgumentInput {
            input: "ride a horse",
            name: "objective",
        }],
    )??;

    assert_eq!(description, Some("Helps with finishing a task".to_owned()));
    assert_eq!(
        messages,
        vec![PromptMessage {
            content: "ride a horse (Your objective, Describe what you are trying to do, true)"
                .into(),
            role: Role::User,
        }]
    );

    Ok(())
}
