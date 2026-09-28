use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_request::mcp_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use serde_json::json;

#[actix_web::test]
async fn rejects_session_request_without_session() {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        mcp_request(&json!({ "id": 1, "jsonrpc": "2.0", "method": "tools/list" })).to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
