use actix_web::http::StatusCode;
use actix_web::http::header::ACCEPT;
use actix_web::test::TestRequest;
use actix_web::test::call_service;
use poet_mcp::mcp_header_protocol_version::MCP_HEADER_PROTOCOL_VERSION;
use poet_mcp::mcp_protocol_version::MCP_PROTOCOL_VERSION;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_test_path::MCP_TEST_PATH;
use poet_mcp_tests::mcp_test_service::mcp_test_service;

#[actix_web::test]
async fn rejects_delete_without_session() {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        TestRequest::delete()
            .uri(MCP_TEST_PATH)
            .insert_header((ACCEPT, "application/json"))
            .insert_header((MCP_HEADER_PROTOCOL_VERSION, MCP_PROTOCOL_VERSION))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
