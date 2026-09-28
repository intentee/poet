use std::sync::Arc;

use actix_web::rt;

use crate::empty_result::EmptyResult;
use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::mcp_error::McpError;
use crate::resource_list_aggregate::ResourceListAggregate;
use crate::resource_update_forwarder::ResourceUpdateForwarder;
use crate::resources_subscribe_request::ResourcesSubscribeRequest;
use crate::resources_subscribe_request_params::ResourcesSubscribeRequestParams;
use crate::server_to_client_response::ServerToClientResponse;
use crate::session::Session;

pub struct ResourcesSubscribeHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

impl ResourcesSubscribeHandler {
    pub fn handle(
        self,
        ResourcesSubscribeRequest {
            id,
            params: ResourcesSubscribeRequestParams { uri, .. },
            ..
        }: ResourcesSubscribeRequest,
        session: &Session,
    ) -> Result<ServerToClientResponse, McpError> {
        let resolved_resource = self.resource_list_aggregate.resolve(&uri)?;
        let cancellation_token = session.subscriptions.subscribe(&uri)?;

        rt::spawn(
            ResourceUpdateForwarder {
                resource_update_notifier: resolved_resource
                    .update_notifier(cancellation_token.clone()),
                cancellation_token,
                session_notifier: session.notifier.clone(),
                uri,
            }
            .forward(),
        );

        Ok(ServerToClientResponse::EmptyResult(
            JsonRpcSuccessResponse::new(id, EmptyResult {}),
        ))
    }
}
