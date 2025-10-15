use serde_json::Value;

use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;

pub enum ToolRegistryCallResult {
    NotFound,
    Success(ToolCallResult<Value>),
}
