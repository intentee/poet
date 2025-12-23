use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;
use log::warn;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::tools_call::ToolsCall;
use crate::mcp::jsonrpc::request::tools_call::ToolsCallParams;
use crate::mcp::jsonrpc::response::error::Error;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::session::Session;
use crate::mcp::tool_registry::ToolRegistry;
use crate::mcp::tool_registry_call_result::ToolRegistryCallResult;

pub struct ToolsCallHandler {
    pub tool_registry: Arc<ToolRegistry>,
}

#[async_trait]
impl Handler for ToolsCallHandler {
    type Request = ToolsCall;
    type Session = Session;

    async fn handle(
        self,
        ToolsCall {
            id,
            params: ToolsCallParams {
                arguments, name, ..
            },
            ..
        }: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let response = match self
            .tool_registry
            .call_tool(&name, arguments)
            .await
            .map_err(ErrorInternalServerError)?
        {
            ToolRegistryCallResult::Success(tool_call_result) => {
                ServerToClientResponse::ToolsCall(Success {
                    id,
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    result: tool_call_result,
                })
            }
            ToolRegistryCallResult::NotFound => {
                warn!("Tool not found: '{name}'");

                ServerToClientResponse::Error(Error::tool_not_found(id, name))
            }
        };

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
