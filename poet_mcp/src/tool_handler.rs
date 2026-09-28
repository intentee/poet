use async_trait::async_trait;
use serde_json::Value;

use crate::mcp_error::McpError;
use crate::tool::Tool;
use crate::tool_call_result::ToolCallResult;

#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn handle(&self, input: Value) -> Result<ToolCallResult<Value>, McpError>;

    fn tool_definition(&self) -> Tool;
}
