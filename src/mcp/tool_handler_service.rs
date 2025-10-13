use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::mcp::tool::Tool;
use crate::mcp::tool_handler::ToolHandler;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_responder::ToolResponder;

pub struct ToolHandlerService<TToolProvider, TToolResponder>
where
    TToolProvider: ToolProvider,
    TToolResponder: ToolResponder<TToolProvider>,
{
    pub provider: Arc<TToolProvider>,
    pub responder: Arc<TToolResponder>,
    pub tool: Tool,
}

#[async_trait]
impl<TToolProvider, TToolResponder> ToolHandler
    for ToolHandlerService<TToolProvider, TToolResponder>
where
    TToolProvider: ToolProvider,
    TToolResponder: ToolResponder<TToolProvider>,
{
    async fn handle(&self, input: Value) -> Result<Value> {
        let input_schema: TToolProvider::InputSchema = serde_json::from_value(input)?;
        let output: TToolProvider::OutputSchema = self.responder.respond(input_schema).await?;

        Ok(serde_json::to_value(output)?)
    }

    fn tool_definition(&self) -> Tool {
        self.tool.clone()
    }
}
