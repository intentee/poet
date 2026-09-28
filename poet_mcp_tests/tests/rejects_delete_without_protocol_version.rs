use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp::mcp_header_protocol_version::MCP_HEADER_PROTOCOL_VERSION;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_delete_request::mcp_delete_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;

#[actix_web::test]
async fn rejects_delete_without_protocol_version() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession { session_id, .. } = InitializedSession::start(&mcp_service).await?;
    let mut delete_request = mcp_delete_request(&session_id).to_request();

    delete_request
        .headers_mut()
        .remove(MCP_HEADER_PROTOCOL_VERSION);

    let response = call_service(&mcp_service, delete_request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
