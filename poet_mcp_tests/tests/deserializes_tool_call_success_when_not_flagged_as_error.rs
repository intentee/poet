use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_call_success::ToolCallSuccess;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use serde_json::from_value;
use serde_json::json;

#[test]
fn deserializes_tool_call_success_when_not_flagged_as_error() -> Result<(), PoetMcpTestsError> {
    let tool_call_result: ToolCallResult<u32> = from_value(json!({
        "content": [],
        "isError": false,
        "structuredContent": 7,
    }))?;

    assert!(matches!(
        tool_call_result,
        ToolCallResult::Success(ToolCallSuccess {
            structured_content: 7,
            ..
        })
    ));

    Ok(())
}
