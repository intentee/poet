use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_request::mcp_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::read_json_response::read_json_response;
use serde_json::json;

#[actix_web::test]
async fn rejects_malformed_message() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        mcp_request(&json!({ "jsonrpc": "2.0", "method": "unknown/method" })).to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response_body = read_json_response(response).await?;

    assert_eq!(response_body["error"]["code"], json!(-32700));
    assert_eq!(response_body["id"], json!(null));
    assert_eq!(response_body["jsonrpc"], json!("2.0"));

    Ok(())
}
