use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize as _;
use serde::Deserializer;
use serde::de::Error as _;

use crate::parse_flexible_datetime::parse_flexible_datetime;

pub fn deserialize_flexible_datetime<'deserialization, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Option<DateTime<Utc>>, TDeserializer::Error>
where
    TDeserializer: Deserializer<'deserialization>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|value| parse_flexible_datetime(&value).map_err(TDeserializer::Error::custom))
        .transpose()
}
