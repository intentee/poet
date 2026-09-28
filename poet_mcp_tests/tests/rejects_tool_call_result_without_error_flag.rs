use poet_mcp::tool_call_result::ToolCallResult;
use serde_json::Value;
use serde_json::from_value;
use serde_json::json;

#[test]
fn rejects_tool_call_result_without_error_flag() {
    assert!(from_value::<ToolCallResult<Value>>(json!({ "content": [] })).is_err());
}
