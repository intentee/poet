use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Argument {
    description: String,
    required: bool,
    title: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptDocumentFrontMatter {
    arguments: HashMap<String, Argument>,
    description: String,
    title: String,
}
