use crate::empty_result::EmptyResult;
use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::ping_request::PingRequest;
use crate::server_to_client_response::ServerToClientResponse;

pub struct PingHandler;

impl PingHandler {
    #[must_use]
    pub fn handle(self, PingRequest { id, .. }: PingRequest) -> ServerToClientResponse {
        ServerToClientResponse::EmptyResult(JsonRpcSuccessResponse::new(id, EmptyResult {}))
    }
}
