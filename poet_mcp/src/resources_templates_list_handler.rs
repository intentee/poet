use std::sync::Arc;

use crate::json_rpc_success_response::JsonRpcSuccessResponse;
use crate::resource_list_aggregate::ResourceListAggregate;
use crate::resources_templates_list_request::ResourcesTemplatesListRequest;
use crate::resources_templates_list_result::ResourcesTemplatesListResult;
use crate::server_to_client_response::ServerToClientResponse;

pub struct ResourcesTemplatesListHandler {
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

impl ResourcesTemplatesListHandler {
    #[must_use]
    pub fn handle(
        self,
        ResourcesTemplatesListRequest { id, .. }: ResourcesTemplatesListRequest,
    ) -> ServerToClientResponse {
        ServerToClientResponse::ResourcesTemplatesList(JsonRpcSuccessResponse::new(
            id,
            ResourcesTemplatesListResult {
                resource_templates: self.resource_list_aggregate.resource_templates(),
            },
        ))
    }
}
