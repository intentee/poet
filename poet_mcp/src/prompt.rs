use serde::Deserialize;
use serde::Serialize;

use crate::prompt_argument::PromptArgument;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Prompt {
    pub arguments: Vec<PromptArgument>,
    pub description: String,
    pub name: String,
    pub title: String,
}
