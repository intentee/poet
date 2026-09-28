use actix_web::test::call_service;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_session_request::mcp_session_request;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use poet_mcp_tests::read_json_response::read_json_response;
use serde_json::json;

#[actix_web::test]
async fn logging_set_level_enables_debug_messages() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession {
        mut event_stream,
        session_id,
        ..
    } = InitializedSession::start(&mcp_service).await?;
    let set_level_response = read_json_response(
        call_service(
            &mcp_service,
            mcp_session_request(
                &session_id,
                &json!({
                    "id": 2,
                    "jsonrpc": "2.0",
                    "method": "logging/setLevel",
                    "params": { "level": "debug" },
                }),
            )
            .to_request(),
        )
        .await,
    )
    .await?;

    assert_eq!(
        set_level_response,
        json!({ "id": 2, "jsonrpc": "2.0", "result": {} })
    );

    call_service(
        &mcp_service,
        mcp_session_request(
            &session_id,
            &json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
        )
        .to_request(),
    )
    .await;

    assert_eq!(
        event_stream.next_event().await?["params"]["level"],
        json!("debug")
    );

    Ok(())
}
