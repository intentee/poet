use async_trait::async_trait;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_call_success::ToolCallSuccess;
use poet_mcp::tool_provider::ToolProvider;
use poet_mcp::tool_responder::ToolResponder;
use serde_json::Value;

use crate::unserializable_tool_output::UnserializableToolOutput;

pub struct UnserializableTool;

impl ToolProvider for UnserializableTool {
    type Input = Value;
    type Output = UnserializableToolOutput;

    fn name(&self) -> String {
        "unserializable".to_owned()
    }
}

#[async_trait]
impl ToolResponder<Self> for UnserializableTool {
    async fn respond(
        &self,
        _input: Value,
    ) -> Result<ToolCallResult<UnserializableToolOutput>, ProviderError> {
        Ok(ToolCallResult::Success(ToolCallSuccess {
            content: vec![],
            structured_content: UnserializableToolOutput,
        }))
    }
}
