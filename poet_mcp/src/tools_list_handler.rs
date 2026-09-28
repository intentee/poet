use std::sync::Arc;

use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::server_to_client_response::ServerToClientResponse;
use crate::tool_registry::ToolRegistry;
use crate::tools_list_request::ToolsListRequest;
use crate::tools_list_request_params::ToolsListRequestParams;
use crate::tools_list_result::ToolsListResult;

pub struct ToolsListHandler {
    pub tool_registry: Arc<ToolRegistry>,
}

impl ToolsListHandler {
    #[must_use]
    pub fn handle(
        self,
        ToolsListRequest {
            id,
            params: ToolsListRequestParams { cursor, .. },
            ..
        }: ToolsListRequest,
    ) -> ServerToClientResponse {
        ServerToClientResponse::ToolsList(JsonRpcSuccessResponse::new(
            id,
            ToolsListResult {
                tools: self
                    .tool_registry
                    .list_tool_definitions(cursor.unwrap_or_default()),
            },
        ))
    }
}
