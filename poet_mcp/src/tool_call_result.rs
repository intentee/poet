use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::DeserializeOwned;
use serde::de::Error as _;
use serde_json::Map;
use serde_json::Value;
use serde_json::to_value;

use crate::content_block::ContentBlock;
use crate::tool_call_error_message::ToolCallErrorMessage;
use crate::tool_call_failure::ToolCallFailure;
use crate::tool_call_success::ToolCallSuccess;

#[derive(Deserialize, Serialize)]
struct FlaggedToolCallResult<TResult> {
    #[serde(flatten)]
    result: TResult,
    #[serde(rename = "isError")]
    is_error: bool,
}

#[derive(Debug)]
pub enum ToolCallResult<TStructuredContent: Serialize> {
    Failure(ToolCallFailure),
    Success(ToolCallSuccess<TStructuredContent>),
}

impl<TStructuredContent: Serialize> ToolCallResult<TStructuredContent> {
    pub fn try_into_value(self) -> Result<ToolCallResult<Value>, serde_json::Error> {
        Ok(match self {
            Self::Failure(failure) => ToolCallResult::Failure(failure),
            Self::Success(ToolCallSuccess {
                content,
                structured_content,
            }) => ToolCallResult::Success(ToolCallSuccess {
                content,
                structured_content: to_value(structured_content)?,
            }),
        })
    }
}

impl<'deserialization, TStructuredContent: DeserializeOwned + Serialize>
    Deserialize<'deserialization> for ToolCallResult<TStructuredContent>
{
    fn deserialize<TDeserializer: Deserializer<'deserialization>>(
        deserializer: TDeserializer,
    ) -> Result<Self, TDeserializer::Error> {
        let FlaggedToolCallResult { result, is_error } =
            FlaggedToolCallResult::<Map<String, Value>>::deserialize(deserializer)?;
        let result_fields = Value::Object(result);

        if is_error {
            ToolCallFailure::deserialize(result_fields)
                .map(Self::Failure)
                .map_err(TDeserializer::Error::custom)
        } else {
            ToolCallSuccess::deserialize(result_fields)
                .map(Self::Success)
                .map_err(TDeserializer::Error::custom)
        }
    }
}

impl<TStructuredContent: Serialize> From<ToolCallErrorMessage<'_>>
    for ToolCallResult<TStructuredContent>
{
    fn from(ToolCallErrorMessage(message): ToolCallErrorMessage<'_>) -> Self {
        Self::Failure(ToolCallFailure {
            content: vec![ContentBlock::from(message)],
        })
    }
}

impl<TStructuredContent: Serialize> Serialize for ToolCallResult<TStructuredContent> {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        match self {
            Self::Failure(failure) => FlaggedToolCallResult {
                result: failure,
                is_error: true,
            }
            .serialize(serializer),
            Self::Success(success) => FlaggedToolCallResult {
                result: success,
                is_error: false,
            }
            .serialize(serializer),
        }
    }
}
