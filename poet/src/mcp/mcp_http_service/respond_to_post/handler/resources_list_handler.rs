use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::resources_list::ResourcesList as ResourcesListRequest;
use crate::mcp::jsonrpc::request::resources_list::ResourcesListParams;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::resources_list::ResourcesList as ResourcesListResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;

pub struct ResourcesListHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

#[async_trait]
impl Handler for ResourcesListHandler {
    type Request = ResourcesListRequest;
    type Session = Session;

    async fn handle(
        self,
        ResourcesListRequest {
            id,
            params: ResourcesListParams { cursor, .. },
            ..
        }: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let list_cursor = match cursor {
            Some(list_cursor) => list_cursor,
            None => ListResourcesCursor::default(),
        };

        if list_cursor.per_page < 1 {
            return Ok(HttpResponse::BadRequest().json(Error::invalid_params(
                id,
                "per_page must be greater than 0".to_string(),
            )));
        }

        let total = self.resource_list_aggregate.total();
        let next_offset = list_cursor.offset.saturating_add(list_cursor.per_page);

        let next_cursor = match next_offset < total {
            true => Some(ListResourcesCursor {
                offset: next_offset,
                per_page: list_cursor.per_page,
            }),
            false => None,
        };

        let response = ServerToClientResponse::ResourcesList(Success {
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: ResourcesListResponse {
                next_cursor,
                resources: self
                    .resource_list_aggregate
                    .list_resources(list_cursor)
                    .await
                    .map_err(ErrorInternalServerError)?,
            },
        });

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
