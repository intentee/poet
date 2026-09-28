use actix_web::http::StatusCode;
use actix_web::http::header::ACCEPT;
use actix_web::test::TestRequest;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_test_path::MCP_TEST_PATH;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use serde_json::json;

#[actix_web::test]
async fn rejects_message_without_protocol_version() {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        TestRequest::post()
            .uri(MCP_TEST_PATH)
            .insert_header((ACCEPT, "application/json, text/event-stream"))
            .set_json(json!({ "id": 1, "jsonrpc": "2.0", "method": "ping" }))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
