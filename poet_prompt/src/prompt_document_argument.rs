use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromptDocumentArgument {
    pub description: String,
    pub required: bool,
    pub title: String,
}
