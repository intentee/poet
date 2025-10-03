use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;

use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevel;
use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevelParams;
use crate::mcp::mcp_http_service::respond_to_post::handler::Handler;
use crate::mcp::session::Session;
use crate::mcp::session_manager::SessionManager;

pub struct LoggingSetLevelHandler {
    pub session_manager: SessionManager,
}

#[async_trait]
impl Handler for LoggingSetLevelHandler {
    type Request = LoggingSetLevel;
    type Session = Session;

    async fn handle(
        self,
        LoggingSetLevel {
            id,
            params: LoggingSetLevelParams { level, .. },
            ..
        }: LoggingSetLevel,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>> {
        self.session_manager
            .update_session(session.clone().with_log_level(level))
            .await?;

        self.empty_response(id, session)
    }
}
