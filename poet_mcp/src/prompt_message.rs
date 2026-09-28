use serde::Deserialize;
use serde::Serialize;

use crate::content_block::ContentBlock;
use crate::role::Role;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptMessage {
    pub content: ContentBlock,
    pub role: Role,
}
