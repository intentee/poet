use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_request::mcp_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::read_json_response::read_json_response;
use serde_json::json;

#[actix_web::test]
async fn answers_ping_without_session() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = read_json_response(
        call_service(
            &mcp_service,
            mcp_request(&json!({ "id": 7, "jsonrpc": "2.0", "method": "ping" })).to_request(),
        )
        .await,
    )
    .await?;

    assert_eq!(response, json!({ "id": 7, "jsonrpc": "2.0", "result": {} }));

    Ok(())
}
