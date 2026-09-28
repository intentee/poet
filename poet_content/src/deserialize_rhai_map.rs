use std::collections::BTreeMap;

use rhai::Dynamic;
use rhai::Map;
use serde::Deserialize as _;
use serde::Deserializer;

pub fn deserialize_rhai_map<'deserialization, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Map, TDeserializer::Error>
where
    TDeserializer: Deserializer<'deserialization>,
{
    BTreeMap::<String, Dynamic>::deserialize(deserializer).map(|entries| {
        entries
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect()
    })
}
