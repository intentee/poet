use std::sync::Arc;

use crate::implementation::Implementation;
use crate::prompt_provider::PromptProvider;
use crate::resource_list_aggregate::ResourceListAggregate;
use crate::session_manager::SessionManager;
use crate::tool_registry::ToolRegistry;

#[derive(Clone)]
pub struct McpServer {
    pub prompt_provider: Arc<dyn PromptProvider>,
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
    pub server_info: Implementation,
    pub session_manager: SessionManager,
    pub tool_registry: Arc<ToolRegistry>,
}
