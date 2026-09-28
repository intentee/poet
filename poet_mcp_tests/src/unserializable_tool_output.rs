use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::Error as _;

#[derive(Deserialize, JsonSchema)]
pub struct UnserializableToolOutput;

impl Serialize for UnserializableToolOutput {
    fn serialize<TSerializer: Serializer>(
        &self,
        _serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        Err(TSerializer::Error::custom(
            "tool output deliberately refuses to serialize",
        ))
    }
}
