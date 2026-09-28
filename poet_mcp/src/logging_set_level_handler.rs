use crate::empty_result::EmptyResult;
use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::logging_set_level_request::LoggingSetLevelRequest;
use crate::logging_set_level_request_params::LoggingSetLevelRequestParams;
use crate::server_to_client_response::ServerToClientResponse;
use crate::session::Session;
use crate::session_manager::SessionManager;

pub struct LoggingSetLevelHandler {
    pub session_manager: SessionManager,
}

impl LoggingSetLevelHandler {
    #[must_use]
    pub fn handle(
        self,
        LoggingSetLevelRequest {
            id,
            params: LoggingSetLevelRequestParams { level, .. },
            ..
        }: LoggingSetLevelRequest,
        session: &Session,
    ) -> ServerToClientResponse {
        self.session_manager.update_session(Session {
            notifier: session.notifier.clone().with_log_level(level),
            ..session.clone()
        });

        ServerToClientResponse::EmptyResult(JsonRpcSuccessResponse::new(id, EmptyResult {}))
    }
}
