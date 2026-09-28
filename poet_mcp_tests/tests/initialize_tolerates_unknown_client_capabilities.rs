use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialize_message::initialize_message;
use poet_mcp_tests::mcp_request::mcp_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use serde_json::json;

#[actix_web::test]
async fn initialize_tolerates_unknown_client_capabilities() {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        mcp_request(&initialize_message(&json!({ "unknownCapability": {} }))).to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
}
