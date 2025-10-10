use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesUpdatedParams {
    pub title: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesUpdated {
    pub jsonrpc: String,
    pub params: ResourcesUpdatedParams,
}
