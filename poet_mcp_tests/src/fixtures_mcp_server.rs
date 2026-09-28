use std::collections::BTreeMap;
use std::sync::Arc;

use poet_mcp::content_block::ContentBlock;
use poet_mcp::mcp_server::McpServer;
use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::resource_provider::ResourceProvider;
use poet_mcp::role::Role;
use poet_mcp::session_manager::SessionManager;
use poet_mcp::tool_registry::ToolRegistry;
use tokio::sync::Notify;

use crate::echo_tool::EchoTool;
use crate::failing_tool::FailingTool;
use crate::fixtures_server_info::fixtures_server_info;
use crate::in_memory_prompt_provider::InMemoryPromptProvider;
use crate::in_memory_resource_provider::InMemoryResourceProvider;
use crate::unserializable_tool::UnserializableTool;

pub struct FixturesMcpServer {
    pub mcp_server: McpServer,
    pub resource_update_notifier: Arc<Notify>,
}

impl Default for FixturesMcpServer {
    fn default() -> Self {
        let resource_update_notifier: Arc<Notify> = Arc::default();
        let resource_providers: Vec<Arc<dyn ResourceProvider>> =
            vec![Arc::new(InMemoryResourceProvider {
                resource_class: "documents".to_owned(),
                resource_texts: BTreeMap::from([
                    ("first".to_owned(), "First document".to_owned()),
                    ("second".to_owned(), "Second document".to_owned()),
                    ("third".to_owned(), "Third document".to_owned()),
                ]),
                resource_update_notifier: resource_update_notifier.clone(),
            })];
        let mut tool_registry = ToolRegistry::default();

        tool_registry.register_owned(EchoTool);
        tool_registry.register_owned(FailingTool);
        tool_registry.register_owned(UnserializableTool);

        Self {
            mcp_server: McpServer {
                prompt_provider: Arc::new(InMemoryPromptProvider {
                    prompt_messages: BTreeMap::from([(
                        "greeting".to_owned(),
                        vec![PromptMessage {
                            content: ContentBlock::from("Say hello"),
                            role: Role::User,
                        }],
                    )]),
                }),
                resource_list_aggregate: Arc::new(resource_providers.into()),
                server_info: fixtures_server_info(),
                session_manager: SessionManager::default(),
                tool_registry: Arc::new(tool_registry),
            },
            resource_update_notifier,
        }
    }
}
