use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct EchoToolInput {
    pub message: String,
}
