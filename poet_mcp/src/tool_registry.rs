use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;

use serde_json::Value;

use crate::list_resources_cursor::ListResourcesCursor;
use crate::mcp_error::McpError;
use crate::tool::Tool;
use crate::tool_call_result::ToolCallResult;
use crate::tool_handler::ToolHandler;
use crate::tool_handler_service::ToolHandlerService;
use crate::tool_provider::ToolProvider;
use crate::tool_responder::ToolResponder;

#[derive(Default)]
pub struct ToolRegistry {
    handlers: BTreeMap<String, Arc<dyn ToolHandler>>,
}

impl ToolRegistry {
    pub async fn call_tool(
        &self,
        tool_name: &str,
        input: Value,
    ) -> Result<ToolCallResult<Value>, McpError> {
        self.handlers
            .get(tool_name)
            .ok_or_else(|| McpError::ToolNotFound {
                tool_name: tool_name.to_owned(),
            })?
            .handle(input)
            .await
    }

    #[must_use]
    pub fn list_tool_definitions(
        &self,
        ListResourcesCursor { offset, per_page }: ListResourcesCursor,
    ) -> Vec<Tool> {
        self.handlers
            .values()
            .skip(offset)
            .take(per_page)
            .map(|tool_handler| tool_handler.tool_definition())
            .collect()
    }

    pub fn register<TToolProvider, TToolResponder>(
        &mut self,
        provider: &Arc<TToolProvider>,
        responder: Arc<TToolResponder>,
    ) where
        TToolProvider: ToolProvider + Send + Sync + 'static,
        TToolResponder: ToolResponder<TToolProvider> + 'static,
    {
        self.handlers.insert(
            provider.name(),
            Arc::new(ToolHandlerService {
                provider: PhantomData,
                responder,
                tool: provider.tool_definition(),
            }),
        );
    }

    pub fn register_owned<TTool>(&mut self, tool: TTool)
    where
        TTool: ToolProvider + ToolResponder<TTool> + Send + Sync + 'static,
    {
        let shared_tool = Arc::new(tool);

        self.register(&shared_tool, shared_tool.clone());
    }
}
