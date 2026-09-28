use async_trait::async_trait;

use crate::list_resources_cursor::ListResourcesCursor;
use crate::prompt::Prompt;
use crate::prompts_get_request::PromptsGetRequest;
use crate::prompts_get_result::PromptsGetResult;
use crate::provider_error::ProviderError;

#[async_trait]
pub trait PromptProvider: Send + Sync {
    async fn get_prompt(
        &self,
        request: PromptsGetRequest,
    ) -> Result<Option<PromptsGetResult>, ProviderError>;

    async fn list_prompts(&self, cursor: ListResourcesCursor)
    -> Result<Vec<Prompt>, ProviderError>;
}
