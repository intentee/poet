use actix_web::App;
use actix_web::http::StatusCode;
use actix_web::test::call_service;
use actix_web::test::init_service;
use actix_web::web::scope;
use poet_mcp::mcp_http_service_factory::McpHttpServiceFactory;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_request::mcp_request;
use poet_mcp_tests::mcp_test_path::MCP_TEST_PATH;
use serde_json::json;

#[actix_web::test]
async fn serves_mcp_mounted_inside_scope() {
    let scoped_service = init_service(App::new().service(scope("/api").service(
        McpHttpServiceFactory {
            mcp_server: FixturesMcpServer::default().mcp_server,
            mount_path: MCP_TEST_PATH.to_owned(),
        },
    )))
    .await;
    let response = call_service(
        &scoped_service,
        mcp_request(&json!({ "id": 1, "jsonrpc": "2.0", "method": "ping" }))
            .uri("/api/mcp")
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
}
