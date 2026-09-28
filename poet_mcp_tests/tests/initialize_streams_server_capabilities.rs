use poet_mcp::mcp_protocol_version::MCP_PROTOCOL_VERSION;
use poet_mcp_tests::fixtures_mcp_server::FixturesMcpServer;
use poet_mcp_tests::initialized_session::InitializedSession;
use poet_mcp_tests::mcp_test_service::mcp_test_service;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use serde_json::json;

#[actix_web::test]
async fn initialize_streams_server_capabilities() -> Result<(), PoetMcpTestsError> {
    let mcp_service = mcp_test_service(FixturesMcpServer::default().mcp_server).await;
    let InitializedSession {
        initialize_result, ..
    } = InitializedSession::start(&mcp_service).await?;

    assert_eq!(
        initialize_result,
        json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "capabilities": {
                    "logging": {},
                    "prompts": { "listChanged": true },
                    "resources": { "listChanged": true, "subscribe": true },
                    "tools": { "listChanged": true },
                },
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "serverInfo": { "name": "fixtures", "title": "Fixtures", "version": "1.0.0" },
            },
        })
    );

    Ok(())
}
