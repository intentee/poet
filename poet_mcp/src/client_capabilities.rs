use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::client_capability_roots::ClientCapabilityRoots;
use crate::empty_object::EmptyObject;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elicitation: Option<EmptyObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<ClientCapabilityRoots>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<EmptyObject>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
