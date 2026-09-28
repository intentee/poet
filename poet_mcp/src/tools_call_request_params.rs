use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::meta::Meta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolsCallRequestParams {
    pub arguments: Value,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub name: String,
}
