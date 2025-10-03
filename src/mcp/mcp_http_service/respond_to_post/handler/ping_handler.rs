use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;

use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::request::ping::Ping;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::empty_response::EmptyResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;

pub struct PingHandler {}

#[async_trait]
impl Handler for PingHandler {
    type Request = Ping;
    type Session = ();

    async fn handle(
        self,
        Ping { id, .. }: Ping,
        _: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        Ok(
            HttpResponse::Ok().json(ServerToClientResponse::EmptyResponse(Success {
                id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: EmptyResponse {},
            })),
        )
    }
}
