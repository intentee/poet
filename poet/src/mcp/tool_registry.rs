use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::Result;
use serde_json::Value;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
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
        ListResourcesCursor { offset, per_page }: ListResourcesCursor,
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

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use schemars::JsonSchema;
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::json;

    use super::*;
    use crate::mcp::content_block::ContentBlock;
    use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
    use crate::mcp::jsonrpc::response::success::tool_call_result::success::Success;

    #[derive(Deserialize, JsonSchema, Serialize)]
    struct EchoInput {
        message: String,
    }

    #[derive(Deserialize, JsonSchema, Serialize)]
    struct EchoOutput {
        echoed: String,
    }

    struct EchoTool {
        tool_name: String,
    }

    impl ToolProvider for EchoTool {
        type Input = EchoInput;
        type Output = EchoOutput;

        fn name(&self) -> String {
            self.tool_name.clone()
        }
    }

    #[async_trait]
    impl ToolResponder<EchoTool> for EchoTool {
        async fn respond(&self, input: EchoInput) -> Result<ToolCallResult<EchoOutput>> {
            Ok(ToolCallResult::Success(Success {
                content: vec![ContentBlock::from(input.message.clone())],
                structured_content: EchoOutput {
                    echoed: input.message,
                },
            }))
        }
    }

    fn registry_with(tool_names: &[&str]) -> ToolRegistry {
        let mut registry = ToolRegistry::default();

        for tool_name in tool_names {
            registry.register_owned(EchoTool {
                tool_name: tool_name.to_string(),
            });
        }

        registry
    }

    #[tokio::test]
    async fn call_tool_routes_input_to_registered_handler() -> Result<()> {
        let ToolRegistryCallResult::Success(ToolCallResult::Success(success)) =
            registry_with(&["echo"])
                .call_tool("echo", json!({ "message": "hello" }))
                .await?
        else {
            panic!("expected a successful tool call");
        };

        assert_eq!(success.structured_content, json!({ "echoed": "hello" }));

        Ok(())
    }

    #[tokio::test]
    async fn call_tool_reports_unknown_tool() -> Result<()> {
        assert!(matches!(
            registry_with(&["echo"])
                .call_tool("missing", json!({ "message": "hello" }))
                .await?,
            ToolRegistryCallResult::NotFound
        ));

        Ok(())
    }

    #[test]
    fn list_tool_definitions_paginates_by_offset_and_per_page() {
        let registry = registry_with(&["echo", "ping"]);

        let first_page = registry.list_tool_definitions(ListResourcesCursor {
            offset: 0,
            per_page: 1,
        });
        let second_page = registry.list_tool_definitions(ListResourcesCursor {
            offset: 1,
            per_page: 1,
        });

        assert_eq!(first_page.len(), 1);
        assert_eq!(first_page[0].name, "echo");
        assert_eq!(second_page.len(), 1);
        assert_eq!(second_page[0].name, "ping");
    }
}
