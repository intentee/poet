use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::mcp::jsonrpc::empty_object::EmptyObject;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::jsonrpc::serde_defaults::default_false;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerCapabilityPrompts {
    #[serde(default = "default_false", rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerCapabilityResources {
    #[serde(default = "default_false", rename = "listChanged")]
    pub list_changed: bool,
    #[serde(default = "default_false")]
    pub subscribe: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerCapabilityTools {
    #[serde(default = "default_false", rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions: Option<EmptyObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<EmptyObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<ServerCapabilityPrompts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ServerCapabilityResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ServerCapabilityTools>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
}
