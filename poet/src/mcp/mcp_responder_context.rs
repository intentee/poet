use actix_web::HttpRequest;
use actix_web::dev::Payload;

use crate::mcp::session::Session;
use crate::mcp::session_manager::SessionManager;

pub struct McpResponderContext {
    pub payload: Payload,
    pub req: HttpRequest,
    pub session: Option<Session>,
    pub session_manager: SessionManager,
}
