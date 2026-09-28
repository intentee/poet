use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerCapabilityResources {
    #[serde(default, rename = "listChanged")]
    pub list_changed: bool,
    #[serde(default)]
    pub subscribe: bool,
}
