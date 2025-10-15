use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::mcp::jsonrpc::content_block::ContentBlock;
use crate::mcp::jsonrpc::content_block::text_content::TextContent;
use crate::mcp::tool_call_error_message::ToolCallErrorMesage;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, tag = "isError")]
pub enum ToolCallResult<TStructuredContent: Serialize> {
    #[serde(rename = "true")]
    Failure { content: Vec<ContentBlock> },
    #[serde(rename = "false")]
    Success {
        content: Vec<ContentBlock>,
        #[serde(rename = "structuredContent")]
        structured_content: TStructuredContent,
    },
}

impl<TStructuredContent: Serialize> ToolCallResult<TStructuredContent> {
    pub fn try_into_value(self) -> Result<ToolCallResult<Value>> {
        match self {
            ToolCallResult::Failure { content } => Ok(ToolCallResult::Failure { content }),
            ToolCallResult::Success {
                content,
                structured_content,
            } => Ok(ToolCallResult::Success {
                content,
                structured_content: serde_json::to_value(structured_content)?,
            }),
        }
    }
}

impl<'a, TStructuredContent: Serialize> From<ToolCallErrorMesage<'a>>
    for ToolCallResult<TStructuredContent>
{
    fn from(message: ToolCallErrorMesage<'a>) -> Self {
        ToolCallResult::Failure {
            content: vec![ContentBlock::TextContent(TextContent {
                text: message.0.to_string(),
            })],
        }
    }
}
