use std::io::Error as IoError;
use std::io::ErrorKind;

use async_trait::async_trait;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_provider::ToolProvider;
use poet_mcp::tool_responder::ToolResponder;
use serde_json::Value;

pub struct FailingTool;

impl ToolProvider for FailingTool {
    type Input = Value;
    type Output = Value;

    fn name(&self) -> String {
        "failing".to_owned()
    }
}

#[async_trait]
impl ToolResponder<Self> for FailingTool {
    async fn respond(&self, _input: Value) -> Result<ToolCallResult<Value>, ProviderError> {
        Err(IoError::from(ErrorKind::NotConnected).into())
    }
}
