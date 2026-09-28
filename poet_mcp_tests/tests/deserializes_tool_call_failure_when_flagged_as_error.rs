use poet_mcp::tool_call_failure::ToolCallFailure;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use serde_json::Value;
use serde_json::from_value;
use serde_json::json;

#[test]
fn deserializes_tool_call_failure_when_flagged_as_error() -> Result<(), PoetMcpTestsError> {
    let tool_call_result: ToolCallResult<Value> =
        from_value(json!({ "content": [], "isError": true }))?;

    assert!(matches!(
        tool_call_result,
        ToolCallResult::Failure(ToolCallFailure { content }) if content.is_empty()
    ));

    Ok(())
}
