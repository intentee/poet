use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::resources_templates_list::ResourcesTemplatesList as ResourcesTemplatesListRequest;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::resource_templates_list::ResourcesTemplatesList as ResourcesTemplatesListResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;

pub struct ResourcesTemplatesListHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

#[async_trait]
impl Handler for ResourcesTemplatesListHandler {
    type Request = ResourcesTemplatesListRequest;
    type Session = Session;

    async fn handle(
        self,
        ResourcesTemplatesListRequest { id, .. }: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let response = ServerToClientResponse::ResourcesTemplatesList(Success {
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: ResourcesTemplatesListResponse {
                resource_templates: self
                    .resource_list_aggregate
                    .read_resources_templates_list()
                    .await
                    .map_err(ErrorInternalServerError)?,
            },
        });

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
