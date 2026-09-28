use serde::Deserialize;
use serde::Serialize;

use crate::prompt::Prompt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptsListResult {
    pub prompts: Vec<Prompt>,
}
