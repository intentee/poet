use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::tool::Tool;
use crate::mcp::tool_handler::ToolHandler;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_responder::ToolResponder;

pub struct ToolHandlerService<TToolProvider, TToolResponder>
where
    TToolProvider: ToolProvider,
    TToolResponder: ToolResponder<TToolProvider>,
{
    pub _provider_phantom: PhantomData<TToolProvider>,
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
    async fn handle(&self, input: Value) -> Result<ToolCallResult<Value>> {
        let input_schema: TToolProvider::Input = serde_json::from_value(input)?;

        Ok(self
            .responder
            .respond(input_schema)
            .await?
            .try_into_value()?)
    }

    fn tool_definition(&self) -> Tool {
        self.tool.clone()
    }
}
