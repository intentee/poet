use async_trait::async_trait;
use poet_mcp::content_block::ContentBlock;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_call_success::ToolCallSuccess;
use poet_mcp::tool_provider::ToolProvider;
use poet_mcp::tool_responder::ToolResponder;

use crate::echo_tool_input::EchoToolInput;
use crate::echo_tool_output::EchoToolOutput;

pub struct EchoTool;

impl ToolProvider for EchoTool {
    type Input = EchoToolInput;
    type Output = EchoToolOutput;

    fn name(&self) -> String {
        "echo".to_owned()
    }
}

#[async_trait]
impl ToolResponder<Self> for EchoTool {
    async fn respond(
        &self,
        EchoToolInput { message }: EchoToolInput,
    ) -> Result<ToolCallResult<EchoToolOutput>, ProviderError> {
        Ok(ToolCallResult::Success(ToolCallSuccess {
            content: vec![ContentBlock::from(message.as_str())],
            structured_content: EchoToolOutput { echoed: message },
        }))
    }
}
