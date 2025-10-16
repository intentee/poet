use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::content_block::ContentBlock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Failure {
    pub content: Vec<ContentBlock>,
}
