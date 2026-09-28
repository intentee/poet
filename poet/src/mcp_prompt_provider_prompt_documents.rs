use std::sync::Arc;

use async_trait::async_trait;
use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt::Prompt;
use poet_mcp::prompt_provider::PromptProvider;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_result::PromptsGetResult;
use poet_mcp::provider_error::ProviderError;

use crate::holder::Holder as _;
use crate::poet_error::PoetError;
use crate::prompt_controller_collection::PromptControllerCollection;
use crate::prompt_controller_collection_holder::PromptControllerCollectionHolder;

pub struct McpPromptProviderPromptDocuments {
    pub prompt_controller_collection_holder: PromptControllerCollectionHolder,
}

impl McpPromptProviderPromptDocuments {
    async fn prompt_controller_collection(
        &self,
    ) -> Result<Arc<PromptControllerCollection>, PoetError> {
        self.prompt_controller_collection_holder
            .get()
            .await
            .ok_or(PoetError::PromptControllerCollectionNotReady)
    }
}

#[async_trait]
impl PromptProvider for McpPromptProviderPromptDocuments {
    async fn get_prompt(
        &self,
        request: PromptsGetRequest,
    ) -> Result<Option<PromptsGetResult>, ProviderError> {
        let prompt_controller_collection = self.prompt_controller_collection().await?;
        let Some(prompt_controller) = prompt_controller_collection.0.get(&request.params.name)
        else {
            return Ok(None);
        };

        Ok(Some(prompt_controller.respond_to(request).await?))
    }

    async fn list_prompts(
        &self,
        cursor: ListResourcesCursor,
    ) -> Result<Vec<Prompt>, ProviderError> {
        Ok(self
            .prompt_controller_collection()
            .await?
            .list_mcp_prompts(cursor))
    }
}
