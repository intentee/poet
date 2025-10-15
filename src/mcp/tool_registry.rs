use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::Result;
use serde_json::Value;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::tool::Tool;
use crate::mcp::tool_handler::ToolHandler;
use crate::mcp::tool_handler_service::ToolHandlerService;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_registry_call_result::ToolRegistryCallResult;
use crate::mcp::tool_responder::ToolResponder;

#[derive(Default)]
pub struct ToolRegistry {
    /// Providers need to be sorted for the offset to work
    handlers: BTreeMap<String, Arc<dyn ToolHandler>>,
}

impl ToolRegistry {
    pub async fn call_tool(&self, tool_name: &str, input: Value) -> Result<ToolRegistryCallResult> {
        match self.handlers.get(tool_name) {
            Some(handler) => handler
                .handle(input)
                .await
                .map(ToolRegistryCallResult::Success),
            None => Ok(ToolRegistryCallResult::NotFound),
        }
    }

    pub fn list_tool_definitions(
        &self,
        ListResourcesParams {
            cursor: ListResourcesCursor { offset },
            per_page,
        }: ListResourcesParams,
    ) -> Vec<Tool> {
        self.handlers
            .values()
            .skip(offset)
            .take(per_page)
            .map(|handler| handler.tool_definition())
            .collect()
    }

    pub fn register<TToolProvider, TToolResponder>(
        &mut self,
        provider: Arc<TToolProvider>,
        responder: Arc<TToolResponder>,
    ) where
        TToolProvider: ToolProvider + Send + Sync + 'static,
        TToolResponder: ToolResponder<TToolProvider> + 'static,
    {
        let name = provider.name();
        let tool_handler_service = ToolHandlerService {
            _provider_phantom: Default::default(),
            responder,
            tool: provider.tool_definition(),
        };

        self.handlers.insert(name, Arc::new(tool_handler_service));
    }

    pub fn register_owned<TTool>(&mut self, tool: TTool)
    where
        TTool: ToolProvider + ToolResponder<TTool> + Send + Sync + 'static,
    {
        let tool_arc = Arc::new(tool);

        self.register(tool_arc.clone(), tool_arc);
    }
}
