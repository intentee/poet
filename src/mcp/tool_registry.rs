use std::collections::HashMap;
use std::sync::Arc;

use crate::mcp::tool::Tool;
use crate::mcp::tool_handler::ToolHandler;
use crate::mcp::tool_handler_service::ToolHandlerService;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_provider::ToolProvider as _;
use crate::mcp::tool_responder::ToolResponder;

#[derive(Default)]
pub struct ToolRegistry {
    pub handlers: HashMap<String, Arc<dyn ToolHandler>>,
}

impl ToolRegistry {
    pub fn list_tool_definitions(&self) -> Vec<Tool> {
        self.handlers
            .values()
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
            tool: provider.tool_definition(),
            provider,
            responder,
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
