use poet_mcp::tool_call_result::ToolCallResult;
use serde_json::Value;
use serde_json::error::Category;
use serde_json::from_value;
use serde_json::json;

#[test]
fn rejects_tool_call_result_without_error_flag() {
    assert!(matches!(
        from_value::<ToolCallResult<Value>>(json!({ "content": [] })),
        Err(json_error) if json_error.classify() == Category::Data
    ));
}
