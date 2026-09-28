use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_session_request::mcp_session_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::read_json_response::read_json_response;
use serde_json::json;

#[actix_web::test]
async fn notifies_about_subscribed_resource_update() -> Result<(), PoetMcpTestsError> {
    let FixturesMcpServer {
        mcp_server,
        resource_update_notifier,
    } = FixturesMcpServer::default();
    let mcp_service = mcp_test_service(mcp_server).await;
    let InitializedSession {
        mut event_stream,
        session_id,
        ..
    } = InitializedSession::start(&mcp_service).await?;
    let subscribe_response = read_json_response(
        call_service(
            &mcp_service,
            mcp_session_request(
                &session_id,
                &json!({
                    "id": 2,
                    "jsonrpc": "2.0",
                    "method": "resources/subscribe",
                    "params": { "uri": "memory://documents/first" },
                }),
            )
            .to_request(),
        )
        .await,
    )
    .await?;

    assert_eq!(
        subscribe_response,
        json!({ "id": 2, "jsonrpc": "2.0", "result": {} })
    );

    resource_update_notifier.notify_one();

    assert_eq!(
        event_stream.next_event().await?,
        json!({
            "jsonrpc": "2.0",
            "method": "notifications/resources/updated",
            "params": { "uri": "memory://documents/first" },
        })
    );

    Ok(())
}
