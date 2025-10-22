pub mod initialize_handler;
pub mod initialized_handler;
pub mod logging_set_level_handler;
pub mod ping_handler;
pub mod prompts_get_handler;
pub mod prompts_list_handler;
pub mod resources_list_handler;
pub mod resources_read_handler;
pub mod resources_subscribe_handler;
pub mod resources_templates_list_handler;
pub mod resources_unsubscribe_handler;
pub mod tools_call_handler;
pub mod tools_list_handler;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use async_trait::async_trait;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::response::success::Success;
use crate::mcp::jsonrpc::response::success::empty_response::EmptyResponse;
use crate::mcp::jsonrpc::server_to_client_response::ServerToClientResponse;
use crate::mcp::session::Session;

#[async_trait]
pub trait Handler: Sized {
    type Request;
    type Session;

    async fn handle(
        self,
        request: Self::Request,
        session: Self::Session,
    ) -> Result<HttpResponse<BoxBody>>;

    fn empty_response(self, id: Id, session: Session) -> Result<HttpResponse<BoxBody>> {
        Ok(HttpResponse::Ok()
            .insert_header((MCP_HEADER_SESSION, session.id()))
            .json(ServerToClientResponse::EmptyResponse(Success {
                id,
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: EmptyResponse {},
            })))
    }
}
