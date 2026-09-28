use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::mcp_delete_request::mcp_delete_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;

#[actix_web::test]
async fn rejects_delete_for_unknown_session() {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let response = call_service(
        &mcp_service,
        mcp_delete_request("poet-unknown").to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
