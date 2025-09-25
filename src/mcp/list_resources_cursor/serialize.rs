use base64::Engine as _;
use base64::engine::general_purpose;
use serde::Serializer;

use crate::mcp::list_resources_cursor::ListResourcesCursor;

pub fn serialize<TSerializer>(
    cursor: &Option<ListResourcesCursor>,
    serializer: TSerializer,
) -> Result<TSerializer::Ok, TSerializer::Error>
where
    TSerializer: Serializer,
{
    match cursor {
        Some(cursor_data) => {
            let json = serde_json::to_vec(cursor_data).map_err(serde::ser::Error::custom)?;
            let base64_string = general_purpose::STANDARD.encode(json);

            serializer.serialize_str(&base64_string)
        }
        None => serializer.serialize_none(),
    }
}
