use std::sync::Arc;

use poet_mcp::mcp_server::McpServer;
use poet_mcp::resource_provider::ResourceProvider;
use poet_mcp::session_manager::SessionManager;
use poet_mcp::tool_registry::ToolRegistry;

use crate::failing_prompt_provider::FailingPromptProvider;
use crate::failing_resource_provider::FailingResourceProvider;
use crate::fixtures_server_info::fixtures_server_info;

#[must_use]
pub fn failing_mcp_server() -> McpServer {
    let resource_providers: Vec<Arc<dyn ResourceProvider>> =
        vec![Arc::new(FailingResourceProvider)];

    McpServer {
        prompt_provider: Arc::new(FailingPromptProvider),
        resource_list_aggregate: Arc::new(resource_providers.into()),
        server_info: fixtures_server_info(),
        session_manager: SessionManager::default(),
        tool_registry: Arc::new(ToolRegistry::default()),
    }
}
