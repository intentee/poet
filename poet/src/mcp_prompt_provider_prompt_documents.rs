use std::sync::Arc;

use async_trait::async_trait;
use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt::Prompt;
use poet_mcp::prompt_provider::PromptProvider;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_result::PromptsGetResult;
use poet_mcp::provider_error::ProviderError;
use poet_prompt::prompt_document_controller_collection::PromptDocumentControllerCollection;

use crate::holder::Holder;
use crate::poet_error::PoetError;

pub struct McpPromptProviderPromptDocuments {
    pub prompt_document_controller_collection_holder:
        Holder<Arc<PromptDocumentControllerCollection>>,
}

impl McpPromptProviderPromptDocuments {
    fn prompt_document_controller_collection(
        &self,
    ) -> Result<Arc<PromptDocumentControllerCollection>, PoetError> {
        self.prompt_document_controller_collection_holder
            .get()
            .ready_or(PoetError::PromptDocumentControllerCollectionNotReady)
    }
}

#[async_trait]
impl PromptProvider for McpPromptProviderPromptDocuments {
    async fn get_prompt(
        &self,
        request: PromptsGetRequest,
    ) -> Result<Option<PromptsGetResult>, ProviderError> {
        let prompt_document_controller_collection = self.prompt_document_controller_collection()?;
        let Some(prompt_document_controller) = prompt_document_controller_collection
            .prompt_document_controllers
            .get(&request.params.name)
        else {
            return Ok(None);
        };

        Ok(Some(prompt_document_controller.respond_to(request)?))
    }

    async fn list_prompts(
        &self,
        cursor: ListResourcesCursor,
    ) -> Result<Vec<Prompt>, ProviderError> {
        Ok(self
            .prompt_document_controller_collection()?
            .list_mcp_prompts(cursor))
    }
}
