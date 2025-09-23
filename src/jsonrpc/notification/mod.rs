pub mod initialized;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Notification<TPayload> {
    pub jsonrpc: String,
    #[serde(flatten)]
    pub payload: TPayload,
}
