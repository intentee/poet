use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(
    deny_unknown_fields,
    rename = "notifications/initialized",
    tag = "method"
)]
pub struct Initialized {
    pub jsonrpc: String,
    pub method: String,
}
