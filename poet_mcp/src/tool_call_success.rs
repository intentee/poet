use serde::Deserialize;
use serde::Serialize;

use crate::content_block::ContentBlock;

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolCallSuccess<TStructuredContent> {
    pub content: Vec<ContentBlock>,
    #[serde(rename = "structuredContent")]
    pub structured_content: TStructuredContent,
}
