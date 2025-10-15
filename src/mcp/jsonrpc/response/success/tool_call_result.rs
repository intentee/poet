use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::content_block::ContentBlock;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCallResult<TStructuredContent> {
    pub content: Vec<ContentBlock>,
    #[serde(default, rename = "isError")]
    pub is_error: bool,
    #[serde(rename = "structuredContent")]
    pub structured_content: TStructuredContent,
}
