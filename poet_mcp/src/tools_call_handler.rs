use std::sync::Arc;

use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::mcp_error::McpError;
use crate::server_to_client_response::ServerToClientResponse;
use crate::tool_registry::ToolRegistry;
use crate::tools_call_request::ToolsCallRequest;
use crate::tools_call_request_params::ToolsCallRequestParams;

pub struct ToolsCallHandler {
    pub tool_registry: Arc<ToolRegistry>,
}

impl ToolsCallHandler {
    pub async fn handle(
        self,
        ToolsCallRequest {
            id,
            params: ToolsCallRequestParams {
                arguments, name, ..
            },
            ..
        }: ToolsCallRequest,
    ) -> Result<ServerToClientResponse, McpError> {
        let tool_call_result = self.tool_registry.call_tool(&name, arguments).await?;

        Ok(ServerToClientResponse::ToolsCall(
            JsonRpcSuccessResponse::new(id, tool_call_result),
        ))
    }
}
