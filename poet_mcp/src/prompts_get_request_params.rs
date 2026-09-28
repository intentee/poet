use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::meta::Meta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptsGetRequestParams {
    pub arguments: HashMap<String, String>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub name: String,
}
