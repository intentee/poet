use std::sync::Arc;

use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::list_resources_cursor::ListResourcesCursor;
use crate::mcp_error::McpError;
use crate::resource_list_aggregate::ResourceListAggregate;
use crate::resources_list_request::ResourcesListRequest;
use crate::resources_list_request_params::ResourcesListRequestParams;
use crate::resources_list_result::ResourcesListResult;
use crate::server_to_client_response::ServerToClientResponse;

pub struct ResourcesListHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

impl ResourcesListHandler {
    pub async fn handle(
        self,
        ResourcesListRequest {
            id,
            params: ResourcesListRequestParams { cursor, .. },
            ..
        }: ResourcesListRequest,
    ) -> Result<ServerToClientResponse, McpError> {
        let list_cursor = cursor.unwrap_or_default();

        if list_cursor.per_page < 1 {
            return Err(McpError::EmptyListPage);
        }

        let next_offset = list_cursor.offset.saturating_add(list_cursor.per_page);
        let next_cursor =
            (next_offset < self.resource_list_aggregate.total()).then_some(ListResourcesCursor {
                offset: next_offset,
                per_page: list_cursor.per_page,
            });
        let resources = self
            .resource_list_aggregate
            .list_resources(list_cursor)
            .await?;

        Ok(ServerToClientResponse::ResourcesList(
            JsonRpcSuccessResponse::new(
                id,
                ResourcesListResult {
                    next_cursor,
                    resources,
                },
            ),
        ))
    }
}
