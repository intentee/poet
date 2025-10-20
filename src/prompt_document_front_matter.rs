use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Argument {
    description: String,
    required: bool,
    title: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptDocumentFrontMatter {
    arguments: HashMap<String, Argument>,
    description: String,
    title: String,
}
