use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::prompts_get_result::PromptsGetResult;
use poet_prompt::prompt_error::PromptError;

use crate::poet_prompt_tests_error::PoetPromptTestsError;
use crate::respond_to_prompt_document::respond_to_prompt_document;

const FIXTURE_FRONT_MATTER: &str = "+++\narguments = {}\ndescription = \"Fixture description\"\ntitle = \"Fixture title\"\n+++\n\n";

pub fn assemble_prompt_messages(
    body: &str,
) -> Result<Result<Vec<PromptMessage>, PromptError>, PoetPromptTestsError> {
    Ok(
        respond_to_prompt_document(&format!("{FIXTURE_FRONT_MATTER}{body}"), &[])?
            .map(|PromptsGetResult { messages, .. }| messages),
    )
}
