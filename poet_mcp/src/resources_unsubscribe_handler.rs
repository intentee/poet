use crate::empty_result::EmptyResult;
use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::resources_unsubscribe_request::ResourcesUnsubscribeRequest;
use crate::resources_unsubscribe_request_params::ResourcesUnsubscribeRequestParams;
use crate::server_to_client_response::ServerToClientResponse;
use crate::session::Session;

pub struct ResourcesUnsubscribeHandler;

impl ResourcesUnsubscribeHandler {
    #[must_use]
    pub fn handle(
        self,
        ResourcesUnsubscribeRequest {
            id,
            params: ResourcesUnsubscribeRequestParams { uri, .. },
            ..
        }: ResourcesUnsubscribeRequest,
        session: &Session,
    ) -> ServerToClientResponse {
        session.subscriptions.unsubscribe(&uri);

        ServerToClientResponse::EmptyResult(JsonRpcSuccessResponse::new(id, EmptyResult {}))
    }
}
