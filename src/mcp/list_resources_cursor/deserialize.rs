use base64::Engine as _;
use base64::engine::general_purpose;
use serde::Deserialize;
use serde::Deserializer;

use crate::mcp::list_resources_cursor::ListResourcesCursor;

pub fn deserialize<'de, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<ListResourcesCursor>, TDeserializer::Error>
where
    TDeserializer: Deserializer<'de>,
{
    let base64_string: Option<String> = Option::deserialize(deserializer)?;

    match base64_string {
        Some(token) => {
            let decoded = general_purpose::STANDARD
                .decode(&token)
                .map_err(serde::de::Error::custom)?;

            let cursor_data: ListResourcesCursor =
                serde_json::from_slice(&decoded).map_err(serde::de::Error::custom)?;

            Ok(Some(cursor_data))
        }
        None => Ok(None),
    }
}
