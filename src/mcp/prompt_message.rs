use serde::Deserialize;
use serde::Serialize;

use crate::mcp::content_block::ContentBlock;
use crate::mcp::jsonrpc::role::Role;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptMessage {
    pub content: ContentBlock,
    pub role: Role,
}
