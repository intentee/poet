use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextContent {
    pub text: String,
}

impl From<&str> for TextContent {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

impl From<String> for TextContent {
    fn from(text: String) -> Self {
        Self { text }
    }
}
