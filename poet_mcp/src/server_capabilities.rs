use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::empty_object::EmptyObject;
use crate::server_capability_prompts::ServerCapabilityPrompts;
use crate::server_capability_resources::ServerCapabilityResources;
use crate::server_capability_tools::ServerCapabilityTools;

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
