use actix_web::HttpRequest;
use actix_web::dev::Payload;

use crate::request_session::RequestSession;

pub struct McpResponderContext {
    pub payload: Payload,
    pub http_request: HttpRequest,
    pub request_session: RequestSession,
}
