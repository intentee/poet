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
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::resources_list::ResourcesList as ResourcesListResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;

const PER_PAGE: usize = 100;

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
        let response = ServerToClientResponse::ResourcesList(Success {
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: ResourcesListResponse {
                resources: self
                    .resource_list_aggregate
                    .list_resources(ListResourcesParams {
                        cursor: cursor.unwrap_or_default(),
                        per_page: PER_PAGE,
                    })
                    .await
                    .map_err(ErrorInternalServerError)?,
            },
        });

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
