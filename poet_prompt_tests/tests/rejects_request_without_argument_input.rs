use poet_prompt::prompt_error::PromptError;
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
fn rejects_request_without_argument_input() -> Result<(), PoetPromptTestsError> {
    assert!(matches!(
        respond_to_prompt_document(DOCUMENT, &[])?,
        Err(PromptError::MissingArgument { name }) if name == "objective"
    ));

    Ok(())
}
