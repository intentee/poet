use actix_http::Request;
use actix_web::App;
use actix_web::Error;
use actix_web::body::BoxBody;
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::test::init_service;
use poet_mcp::mcp_http_service_factory::McpHttpServiceFactory;
use poet_mcp::mcp_server::McpServer;

use crate::mcp_test_path::MCP_TEST_PATH;

pub async fn mcp_test_service(
    mcp_server: McpServer,
) -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = Error> {
    init_service(App::new().service(McpHttpServiceFactory {
        mcp_server,
        mount_path: MCP_TEST_PATH.to_owned(),
    }))
    .await
}
