use actix_web::http::StatusCode;
use actix_web::http::header::ACCEPT;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_request::mcp_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::read_json_response::read_json_response;
use serde_json::json;

#[actix_web::test]
async fn rejects_post_without_acceptable_mime() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        mcp_request(&json!({ "id": 1, "jsonrpc": "2.0", "method": "ping" }))
            .insert_header((ACCEPT, "application/json"))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
    assert_eq!(
        read_json_response(response).await?["error"]["code"],
        json!(-32600)
    );

    Ok(())
}
