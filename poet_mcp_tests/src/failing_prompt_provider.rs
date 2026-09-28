use std::io::Error as IoError;
use std::io::ErrorKind;

use async_trait::async_trait;
use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt::Prompt;
use poet_mcp::prompt_provider::PromptProvider;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_result::PromptsGetResult;
use poet_mcp::provider_error::ProviderError;

pub struct FailingPromptProvider;

#[async_trait]
impl PromptProvider for FailingPromptProvider {
    async fn get_prompt(
        &self,
        _request: PromptsGetRequest,
    ) -> Result<Option<PromptsGetResult>, ProviderError> {
        Err(IoError::from(ErrorKind::NotConnected).into())
    }

    async fn list_prompts(
        &self,
        _cursor: ListResourcesCursor,
    ) -> Result<Vec<Prompt>, ProviderError> {
        Err(IoError::from(ErrorKind::NotConnected).into())
    }
}
