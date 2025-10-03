use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::error::ErrorInternalServerError;
use async_trait::async_trait;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::notification::initialized::Initialized;
use crate::mcp::jsonrpc::notification::message::MessageParams;
use crate::mcp::log_level::LogLevel;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::session::Session;

pub struct InitializedHandler {}

#[async_trait]
impl Handler for InitializedHandler {
    type Request = Initialized;
    type Session = Session;

    async fn handle(
        self,
        _: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        session
            .log_message(MessageParams {
                data: "Initialization handshake is successfully completed".to_string(),
                level: LogLevel::Debug,
            })
            .await
            .map_err(ErrorInternalServerError)?;

        Ok(HttpResponse::Accepted()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .finish())
    }
}
