use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PromptArgument {
    pub description: String,
    pub name: String,
    #[serde(default)]
    pub required: bool,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Prompt {
    pub arguments: Vec<PromptArgument>,
    pub description: String,
    pub name: String,
    pub title: String,
}
