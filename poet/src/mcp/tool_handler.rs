use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::tool::Tool;

#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn handle(&self, input: Value) -> Result<ToolCallResult<Value>>;

    fn tool_definition(&self) -> Tool;
}
