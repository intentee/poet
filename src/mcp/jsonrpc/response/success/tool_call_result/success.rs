use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::content_block::ContentBlock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Success<TStructuredContent: Serialize> {
    pub content: Vec<ContentBlock>,
    #[serde(rename = "structuredContent")]
    pub structured_content: TStructuredContent,
}
