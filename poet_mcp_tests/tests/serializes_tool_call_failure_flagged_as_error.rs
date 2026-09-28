use poet_mcp::tool_call_error_message::ToolCallErrorMessage;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;
use serde_json::Value;
use serde_json::json;
use serde_json::to_value;

#[test]
fn serializes_tool_call_failure_flagged_as_error() -> Result<(), PoetMcpTestsError> {
    let tool_call_result: ToolCallResult<Value> = ToolCallErrorMessage("boom").into();

    assert_eq!(
        to_value(tool_call_result)?,
        json!({
            "content": [{ "text": "boom", "type": "text" }],
            "isError": true,
        })
    );

    Ok(())
}
