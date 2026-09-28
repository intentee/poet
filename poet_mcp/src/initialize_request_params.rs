use serde::Deserialize;
use serde::Serialize;

use crate::client_capabilities::ClientCapabilities;
use crate::implementation::Implementation;
use crate::meta::Meta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitializeRequestParams {
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}
