use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp::mcp_header_session::MCP_HEADER_SESSION;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_session_request::mcp_session_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use serde_json::json;

#[actix_web::test]
async fn acknowledges_initialized_notification() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession { session_id, .. } = InitializedSession::start(&mcp_service).await?;
    let response = call_service(
        &mcp_service,
        mcp_session_request(
            &session_id,
            &json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
        )
        .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::ACCEPTED);
    assert_eq!(
        response.headers().get(MCP_HEADER_SESSION),
        Some(&session_id.parse()?)
    );

    Ok(())
}
