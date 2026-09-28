use actix_web::body::BoxBody;
use actix_web::dev::AppService;
use actix_web::dev::HttpServiceFactory;
use actix_web::dev::ResourceDef;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::error::Error;
use futures_util::future::LocalBoxFuture;

use crate::mcp_http_service::McpHttpService;
use crate::mcp_server::McpServer;

pub struct McpHttpServiceFactory {
    pub mcp_server: McpServer,
    pub mount_path: String,
}

impl ServiceFactory<ServiceRequest> for McpHttpServiceFactory {
    type Config = ();
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;
    type InitError = ();
    type Response = ServiceResponse<BoxBody>;
    type Service = McpHttpService;

    fn new_service(&self, (): Self::Config) -> Self::Future {
        let mcp_server = self.mcp_server.clone();

        Box::pin(async move { Ok(McpHttpService { mcp_server }) })
    }
}

impl HttpServiceFactory for McpHttpServiceFactory {
    fn register(self, config: &mut AppService) {
        let resource_definition = if config.is_root() {
            ResourceDef::root_prefix(&self.mount_path)
        } else {
            ResourceDef::prefix(&self.mount_path)
        };

        config.register_service(resource_definition, None, self, None);
    }
}
