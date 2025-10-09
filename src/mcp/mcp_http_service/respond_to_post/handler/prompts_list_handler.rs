use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::prompts_list::PromptsList as PromptsListRequest;
use crate::mcp::jsonrpc::request::prompts_list::PromptsListParams;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::prompts_list::PromptsList as PromptsListResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::session::Session;

pub struct PromptsListHandler {}

#[async_trait]
impl Handler for PromptsListHandler {
    type Request = PromptsListRequest;
    type Session = Session;

    async fn handle(
        self,
        PromptsListRequest {
            id,
            params: PromptsListParams { cursor, .. },
            ..
        }: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        let response = ServerToClientResponse::PromptsList(Success {
            id,
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: PromptsListResponse { prompts: vec![] },
        });

        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(response))
    }
}
