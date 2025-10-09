use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;
use log::warn;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::resources_read::ResourcesRead as ResourcesReadRequest;
use crate::mcp::jsonrpc::request::resources_read::ResourcesReadParams;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::resources_read::ResourcesRead as ResourcesReadResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session::Session;

pub struct ResourcesReadHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

#[async_trait]
impl Handler for ResourcesReadHandler {
    type Request = ResourcesReadRequest;
    type Session = Session;

    async fn handle(
        self,
        ResourcesReadRequest {
            id,
            params: ResourcesReadParams { uri, .. },
            ..
        }: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let response = match self
            .resource_list_aggregate
            .read_resource_contents(&uri)
            .await
            .map_err(ErrorInternalServerError)?
        {
            Some(ResourceContentParts {
                parts: contents, ..
            }) => ServerToClientResponse::ResourcesRead(Success {
                id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: ResourcesReadResponse { contents },
            }),
            None => {
                warn!("Resource not found: '{uri}'");

                ServerToClientResponse::Error(Error::resource_not_found(id, uri))
            }
        };

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
