use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use serde_json::from_value;

use crate::mcp_error::McpError;
use crate::tool::Tool;
use crate::tool_call_result::ToolCallResult;
use crate::tool_handler::ToolHandler;
use crate::tool_provider::ToolProvider;
use crate::tool_responder::ToolResponder;

pub struct ToolHandlerService<TToolProvider, TToolResponder>
where
    TToolProvider: ToolProvider,
    TToolResponder: ToolResponder<TToolProvider>,
{
    pub provider: PhantomData<TToolProvider>,
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
    async fn handle(&self, input: Value) -> Result<ToolCallResult<Value>, McpError> {
        let tool_input: TToolProvider::Input =
            from_value(input).map_err(|source| McpError::DeserializeToolInput {
                tool_name: self.tool.name.clone(),
                source,
            })?;

        self.responder
            .respond(tool_input)
            .await
            .map_err(|source| McpError::ToolFailed {
                tool_name: self.tool.name.clone(),
                source,
            })?
            .try_into_value()
            .map_err(|source| McpError::SerializeToolOutput {
                tool_name: self.tool.name.clone(),
                source,
            })
    }

    fn tool_definition(&self) -> Tool {
        self.tool.clone()
    }
}
