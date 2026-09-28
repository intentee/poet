use actix_web::http::StatusCode;
use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_delete_request::mcp_delete_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;

#[actix_web::test]
async fn terminating_session_ends_its_event_stream() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession {
        event_stream,
        session_id,
        ..
    } = InitializedSession::start(&mcp_service).await?;
    let delete_response =
        call_service(&mcp_service, mcp_delete_request(&session_id).to_request()).await;

    assert_eq!(delete_response.status(), StatusCode::ACCEPTED);

    event_stream.ended().await
}
