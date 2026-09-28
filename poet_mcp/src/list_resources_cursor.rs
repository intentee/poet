use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use serde_json::from_slice;
use serde_json::json;

use crate::list_resources_cursor_token::ListResourcesCursorToken;

const DEFAULT_PER_PAGE: usize = 20;

#[derive(Debug, Eq, PartialEq)]
pub struct ListResourcesCursor {
    pub offset: usize,
    pub per_page: usize,
}

impl Default for ListResourcesCursor {
    fn default() -> Self {
        Self {
            offset: 0,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl<'deserialization> Deserialize<'deserialization> for ListResourcesCursor {
    fn deserialize<TDeserializer: Deserializer<'deserialization>>(
        deserializer: TDeserializer,
    ) -> Result<Self, TDeserializer::Error> {
        let encoded_token = String::deserialize(deserializer)?;
        let decoded_token = STANDARD
            .decode(encoded_token)
            .map_err(TDeserializer::Error::custom)?;
        let ListResourcesCursorToken { offset, per_page } =
            from_slice(&decoded_token).map_err(TDeserializer::Error::custom)?;

        Ok(Self { offset, per_page })
    }
}

impl Serialize for ListResourcesCursor {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        let decoded_token = json!({
            "offset": self.offset,
            "per_page": self.per_page,
        });

        serializer.serialize_str(&STANDARD.encode(decoded_token.to_string()))
    }
}
