use serde::Deserialize;
use serde::Serialize;

use crate::mcp::prompt::Prompt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptsList {
    pub prompts: Vec<Prompt>,
}
