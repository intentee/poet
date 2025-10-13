use schemars::Schema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputSchema")]
    pub input_schema: Schema,
    pub name: String,
    #[serde(rename = "outputSchema")]
    pub output_schema: Schema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
