use std::sync::Arc;

use actix_web::body::BoxBody;
use actix_web::dev::AppService;
use actix_web::dev::HttpServiceFactory;
use actix_web::dev::ResourceDef;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::error::Error;
use futures_util::future::LocalBoxFuture;

use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::mcp_http_service::McpHttpService;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session_manager::SessionManager;
use crate::mcp::tool_registry::ToolRegistry;

pub struct McpHttpServiceFactory {
    pub mount_path: String,
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
    pub server_info: Implementation,
    pub session_manager: SessionManager,
    pub tool_registry: Arc<ToolRegistry>,
}

impl ServiceFactory<ServiceRequest> for McpHttpServiceFactory {
    type Config = ();
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<BoxBody>;
    type Service = McpHttpService;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let resource_list_aggregate = self.resource_list_aggregate.clone();
        let server_info = self.server_info.clone();
        let session_manager = self.session_manager.clone();
        let tool_registry = self.tool_registry.clone();

        Box::pin(async move {
            Ok(McpHttpService {
                resource_list_aggregate,
                server_info,
                session_manager,
                tool_registry,
            })
        })
    }
}

impl HttpServiceFactory for McpHttpServiceFactory {
    fn register(self, config: &mut AppService) {
        config.register_service(
            if config.is_root() {
                ResourceDef::root_prefix(&self.mount_path)
            } else {
                ResourceDef::prefix(&self.mount_path)
            },
            None,
            self,
            None,
        );
    }
}
