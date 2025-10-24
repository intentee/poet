use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextContent {
    pub text: String,
}

impl From<&str> for TextContent {
    fn from(value: &str) -> Self {
        Self {
            text: value.to_string(),
        }
    }
}

impl From<String> for TextContent {
    fn from(text: String) -> Self {
        Self { text }
    }
}
