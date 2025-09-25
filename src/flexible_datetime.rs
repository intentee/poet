use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;
use serde::de::Error;

const DATETIME_FORMAT_PATTERNS: &[&str] = &[
    "%Y-%m-%d %H:%M:%S%.f",
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%d",
    "%Y/%m/%d %H:%M:%S",
    "%Y/%m/%d %H:%M",
    "%Y/%m/%d",
    "%d-%m-%Y %H:%M:%S",
    "%d-%m-%Y %H:%M",
    "%d-%m-%Y",
    "%Y-%m-%dT%H:%M:%S%.fZ",
    "%Y-%m-%dT%H:%M:%SZ",
    "%Y-%m-%dT%H:%M:%S",
];

pub fn serialize<TSerializer>(
    datetime_value: &DateTime<Utc>,
    serializer: TSerializer,
) -> Result<TSerializer::Ok, TSerializer::Error>
where
    TSerializer: Serializer,
{
    serializer.serialize_str(&datetime_value.to_rfc3339())
}

pub fn deserialize<'deserialization_lifetime, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<DateTime<Utc>, TDeserializer::Error>
where
    TDeserializer: Deserializer<'deserialization_lifetime>,
{
    let input_string = String::deserialize(deserializer)?;

    for datetime_format_pattern in DATETIME_FORMAT_PATTERNS {
        if let Ok(parsed_naive_datetime) =
            NaiveDateTime::parse_from_str(&input_string, datetime_format_pattern)
        {
            return Ok(DateTime::from_naive_utc_and_offset(
                parsed_naive_datetime,
                Utc,
            ));
        }

        if let Ok(parsed_naive_date) =
            NaiveDate::parse_from_str(&input_string, datetime_format_pattern)
        {
            let naive_datetime_at_midnight = parsed_naive_date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| TDeserializer::Error::custom("Invalid time"))?;
            return Ok(DateTime::from_naive_utc_and_offset(
                naive_datetime_at_midnight,
                Utc,
            ));
        }
    }

    if let Ok(rfc3339_parsed_datetime) = DateTime::parse_from_rfc3339(&input_string) {
        return Ok(rfc3339_parsed_datetime.with_timezone(&Utc));
    }

    if let Ok(rfc2822_parsed_datetime) = DateTime::parse_from_rfc2822(&input_string) {
        return Ok(rfc2822_parsed_datetime.with_timezone(&Utc));
    }

    Err(TDeserializer::Error::custom(format!(
        "Unable to parse '{}' as datetime with any known format",
        input_string
    )))
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(with = "crate::flexible_datetime")]
        timestamp: DateTime<Utc>,
    }

    #[test]
    fn test_deserialize_date_only_converts_to_midnight() {
        let input_json = r#"{"timestamp":"2025-09-25"}"#;
        let deserialized: TestStruct = serde_json::from_str(input_json).unwrap();
        let expected_datetime = DateTime::parse_from_rfc3339("2025-09-25T00:00:00+00:00")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(deserialized.timestamp, expected_datetime);
    }

    #[test]
    fn test_serialize_always_outputs_rfc3339() {
        let test_struct = TestStruct {
            timestamp: DateTime::parse_from_rfc3339("2025-09-25T14:30:45+00:00")
                .unwrap()
                .with_timezone(&Utc),
        };
        let serialized = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(serialized, r#"{"timestamp":"2025-09-25T14:30:45+00:00"}"#);
    }
}
