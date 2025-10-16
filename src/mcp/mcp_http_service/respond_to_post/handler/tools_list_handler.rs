use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::tools_list::ToolsList as ToolsListRequest;
use crate::mcp::jsonrpc::request::tools_list::ToolsListParams;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::tools_list::ToolsList as ToolsListResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::session::Session;
use crate::mcp::tool_registry::ToolRegistry;

pub struct ToolsListHandler {
    pub tool_registry: Arc<ToolRegistry>,
}

#[async_trait]
impl Handler for ToolsListHandler {
    type Request = ToolsListRequest;
    type Session = Session;

    async fn handle(
        self,
        ToolsListRequest {
            id,
            params: ToolsListParams { cursor, .. },
            ..
        }: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let response = ServerToClientResponse::ToolsList(Success {
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: ToolsListResponse {
                tools: self
                    .tool_registry
                    .list_tool_definitions(cursor.unwrap_or_default()),
            },
        });

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
