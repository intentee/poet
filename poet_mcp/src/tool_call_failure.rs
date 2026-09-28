use serde::Deserialize;
use serde::Serialize;

use crate::content_block::ContentBlock;

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolCallFailure {
    pub content: Vec<ContentBlock>,
}
