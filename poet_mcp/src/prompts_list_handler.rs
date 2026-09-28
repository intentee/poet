use std::sync::Arc;

use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::mcp_error::McpError;
use crate::prompt_provider::PromptProvider;
use crate::prompts_list_request::PromptsListRequest;
use crate::prompts_list_request_params::PromptsListRequestParams;
use crate::prompts_list_result::PromptsListResult;
use crate::server_to_client_response::ServerToClientResponse;

pub struct PromptsListHandler {
    pub prompt_provider: Arc<dyn PromptProvider>,
}

impl PromptsListHandler {
    pub async fn handle(
        self,
        PromptsListRequest {
            id,
            params: PromptsListRequestParams { cursor, .. },
            ..
        }: PromptsListRequest,
    ) -> Result<ServerToClientResponse, McpError> {
        let prompts = self
            .prompt_provider
            .list_prompts(cursor.unwrap_or_default())
            .await
            .map_err(|source| McpError::PromptProviderFailed { source })?;

        Ok(ServerToClientResponse::PromptsList(
            JsonRpcSuccessResponse::new(id, PromptsListResult { prompts }),
        ))
    }
}
