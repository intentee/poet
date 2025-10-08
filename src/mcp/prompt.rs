use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PromptArgument {
    description: String,
    name: String,
    #[serde(default)]
    required: bool,
    title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Prompt {
    pub arguments: Vec<PromptArgument>,
    pub description: String,
    pub name: String,
    pub title: String,
}
