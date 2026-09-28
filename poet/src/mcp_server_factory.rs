use std::sync::Arc;

use poet_mcp::implementation::Implementation;
use poet_mcp::mcp_server::McpServer;
use poet_mcp::resource_provider::ResourceProvider;
use poet_mcp::session_manager::SessionManager;
use poet_mcp::tool_registry::ToolRegistry;

use crate::build_project_result_holder::BuildProjectResultHolder;
use crate::mcp_prompt_provider_prompt_documents::McpPromptProviderPromptDocuments;
use crate::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use crate::prompt_controller_collection_holder::PromptControllerCollectionHolder;
use crate::search_index_reader_holder::SearchIndexReaderHolder;
use crate::search_tool::SearchTool;

pub struct McpServerFactory {
    pub build_project_result_holder: BuildProjectResultHolder,
    pub prompt_controller_collection_holder: PromptControllerCollectionHolder,
    pub search_index_reader_holder: SearchIndexReaderHolder,
    pub server_info: Implementation,
}

impl McpServerFactory {
    pub fn create(self) -> McpServer {
        let mcp_resource_provider_content_documents = McpResourceProviderContentDocuments {
            build_project_result_holder: self.build_project_result_holder,
        };
        let resource_providers: Vec<Arc<dyn ResourceProvider>> =
            vec![Arc::new(mcp_resource_provider_content_documents.clone())];
        let mut tool_registry = ToolRegistry::default();

        tool_registry.register_owned(SearchTool {
            mcp_resource_provider_content_documents,
            search_index_reader_holder: self.search_index_reader_holder,
        });

        McpServer {
            prompt_provider: Arc::new(McpPromptProviderPromptDocuments {
                prompt_controller_collection_holder: self.prompt_controller_collection_holder,
            }),
            resource_list_aggregate: Arc::new(resource_providers.into()),
            server_info: self.server_info,
            session_manager: SessionManager::default(),
            tool_registry: Arc::new(tool_registry),
        }
    }
}
