use std::sync::Arc;

use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::mcp_error::McpError;
use crate::resource_list_aggregate::ResourceListAggregate;
use crate::resources_read_request::ResourcesReadRequest;
use crate::resources_read_request_params::ResourcesReadRequestParams;
use crate::resources_read_result::ResourcesReadResult;
use crate::server_to_client_response::ServerToClientResponse;

pub struct ResourcesReadHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

impl ResourcesReadHandler {
    pub async fn handle(
        self,
        ResourcesReadRequest {
            id,
            params: ResourcesReadRequestParams { uri, .. },
            ..
        }: ResourcesReadRequest,
    ) -> Result<ServerToClientResponse, McpError> {
        let contents = self
            .resource_list_aggregate
            .resolve(&uri)?
            .read_contents()
            .await?;

        Ok(ServerToClientResponse::ResourcesRead(
            JsonRpcSuccessResponse::new(
                id,
                ResourcesReadResult {
                    contents,
                    meta: None,
                },
            ),
        ))
    }
}
