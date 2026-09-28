use actix_web::http::StatusCode;
use actix_web::http::header::ALLOW;
use actix_web::http::header::HeaderValue;
use actix_web::test::TestRequest;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_test_path::MCP_TEST_PATH;
use poet_mcp_tests::mcp_test_service::mcp_test_service;

#[actix_web::test]
async fn rejects_get_with_allowed_methods() {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        TestRequest::get().uri(MCP_TEST_PATH).to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(
        response.headers().get(ALLOW),
        Some(&HeaderValue::from_static("DELETE, POST"))
    );
}
