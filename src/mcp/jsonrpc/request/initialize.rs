use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::mcp::jsonrpc::empty_object::EmptyObject;
use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::jsonrpc::meta::Meta;
use crate::mcp::jsonrpc::serde_defaults::default_false;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientCapabilityRoots {
    #[serde(default = "default_false", rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elicitation: Option<EmptyObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<ClientCapabilityRoots>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<EmptyObject>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitializeParams {
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename = "initialize", tag = "method")]
pub struct Initialize {
    pub id: Id,
    pub jsonrpc: String,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub method: String,
    pub params: InitializeParams,
}
