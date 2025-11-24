pub mod failure;
pub mod success;

use anyhow::Result;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de;
use serde::de::DeserializeOwned;
use serde::ser;
use serde_json::Map;
use serde_json::Value;

use crate::mcp::content_block::ContentBlock;
use crate::mcp::content_block::text_content::TextContent;
use crate::mcp::jsonrpc::response::success::tool_call_result::failure::Failure;
use crate::mcp::jsonrpc::response::success::tool_call_result::success::Success;
use crate::mcp::tool_call_error_message::ToolCallErrorMesage;

#[derive(Debug)]
pub enum ToolCallResult<TStructuredContent: Serialize> {
    Failure(Failure),
    Success(Success<TStructuredContent>),
}

impl<TStructuredContent: Serialize> ToolCallResult<TStructuredContent> {
    pub fn try_into_value(self) -> Result<ToolCallResult<Value>> {
        match self {
            ToolCallResult::Failure(failure) => Ok(ToolCallResult::Failure(failure)),
            ToolCallResult::Success(Success {
                content,
                structured_content,
            }) => Ok(ToolCallResult::Success(Success {
                content,
                structured_content: serde_json::to_value(structured_content)?,
            })),
        }
    }
}

impl<'de, TStructuredContent: DeserializeOwned + Serialize> Deserialize<'de>
    for ToolCallResult<TStructuredContent>
{
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> Result<ToolCallResult<TStructuredContent>, TDeserializer::Error> {
        let mut map = Map::deserialize(deserializer)?;
        let is_error_value: Value = map
            .remove("isError")
            .ok_or_else(|| de::Error::missing_field("isError"))?;

        let is_error: bool = is_error_value
            .as_bool()
            .ok_or_else(|| de::Error::custom("'isError' field must be a 'bool'"))?;

        let rest = Value::Object(map);

        if is_error {
            Failure::deserialize(rest)
                .map(ToolCallResult::Failure)
                .map_err(de::Error::custom)
        } else {
            Success::deserialize(rest)
                .map(ToolCallResult::Success)
                .map_err(de::Error::custom)
        }
    }
}

impl<'a, TStructuredContent: Serialize> From<ToolCallErrorMesage<'a>>
    for ToolCallResult<TStructuredContent>
{
    fn from(message: ToolCallErrorMesage<'a>) -> Self {
        ToolCallResult::Failure(Failure {
            content: vec![ContentBlock::TextContent(TextContent {
                text: message.0.to_string(),
            })],
        })
    }
}

impl<TStructuredContent: Serialize> Serialize for ToolCallResult<TStructuredContent> {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        let (is_error, mut map) = match self {
            ToolCallResult::Failure(failure) => {
                let value = serde_json::to_value(failure).map_err(ser::Error::custom)?;

                (
                    true,
                    value
                        .as_object()
                        .ok_or_else(|| ser::Error::custom("Failure must serialize to an object"))?
                        .clone(),
                )
            }
            ToolCallResult::Success(success) => {
                let value = serde_json::to_value(success).map_err(ser::Error::custom)?;

                (
                    false,
                    value
                        .as_object()
                        .ok_or_else(|| ser::Error::custom("Success must serialize to an object"))?
                        .clone(),
                )
            }
        };

        map.insert("isError".to_string(), Value::Bool(is_error));

        map.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let serialized_correct = r#"{"content":[],"isError":true}"#;

        let tool_call_result: ToolCallResult<()> =
            serde_json::from_str(serialized_correct).unwrap();

        assert!(matches!(tool_call_result, ToolCallResult::Failure(_)));

        let serialized = serde_json::to_string(&tool_call_result).unwrap();

        assert_eq!(serialized, serialized_correct);
    }
}
