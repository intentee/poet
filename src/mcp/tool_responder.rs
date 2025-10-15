use anyhow::Result;
use async_trait::async_trait;

use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::tool_provider::ToolProvider;

#[async_trait]
pub trait ToolResponder<TToolProvider>
where
    Self: Send + Sync,
    TToolProvider: ToolProvider,
{
    async fn respond(
        &self,
        input: <TToolProvider as ToolProvider>::InputSchema,
    ) -> Result<ToolCallResult<<TToolProvider as ToolProvider>::OutputSchema>>;
}
