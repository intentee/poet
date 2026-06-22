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

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(deserialize_with = "deserialize")]
        cursor: Option<ListResourcesCursor>,
    }

    fn parse(json: &str) -> Result<Wrapper, serde_json::Error> {
        serde_json::from_str(json)
    }

    #[test]
    fn returns_none_for_null_token() -> Result<(), serde_json::Error> {
        let wrapper = parse("{\"cursor\": null}")?;

        assert!(wrapper.cursor.is_none());

        Ok(())
    }

    #[test]
    fn decodes_offset_and_per_page_from_valid_token() -> Result<(), serde_json::Error> {
        let token = general_purpose::STANDARD.encode("{\"offset\":5,\"per_page\":10}");
        let wrapper = parse(&format!("{{\"cursor\": \"{token}\"}}"))?;

        assert_eq!(wrapper.cursor.as_ref().map(|cursor| cursor.offset), Some(5));
        assert_eq!(
            wrapper.cursor.as_ref().map(|cursor| cursor.per_page),
            Some(10)
        );

        Ok(())
    }

    #[test]
    fn fails_for_token_that_is_not_valid_base64() {
        assert!(parse("{\"cursor\": \"@@@invalid@@@\"}").is_err());
    }

    #[test]
    fn fails_for_token_whose_decoded_bytes_are_not_valid_cursor_json() {
        let token = general_purpose::STANDARD.encode("not json");

        assert!(parse(&format!("{{\"cursor\": \"{token}\"}}")).is_err());
    }
}
