use std::sync::Arc;

use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::mcp_error::McpError;
use crate::prompt_provider::PromptProvider;
use crate::prompts_get_request::PromptsGetRequest;
use crate::server_to_client_response::ServerToClientResponse;

pub struct PromptsGetHandler {
    pub prompt_provider: Arc<dyn PromptProvider>,
}

impl PromptsGetHandler {
    pub async fn handle(
        self,
        prompts_get_request: PromptsGetRequest,
    ) -> Result<ServerToClientResponse, McpError> {
        let request_id = prompts_get_request.id.clone();
        let prompt_name = prompts_get_request.params.name.clone();
        let prompts_get_result = self
            .prompt_provider
            .get_prompt(prompts_get_request)
            .await
            .map_err(|source| McpError::PromptProviderFailed { source })?
            .ok_or(McpError::PromptNotFound { prompt_name })?;

        Ok(ServerToClientResponse::PromptsGet(
            JsonRpcSuccessResponse::new(request_id, prompts_get_result),
        ))
    }
}
