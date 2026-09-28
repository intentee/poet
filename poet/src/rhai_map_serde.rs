use std::collections::BTreeMap;

use rhai::Dynamic;
use rhai::Map;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;

pub fn serialize<TSerializer>(
    map: &Map,
    serializer: TSerializer,
) -> Result<TSerializer::Ok, TSerializer::Error>
where
    TSerializer: Serializer,
{
    serializer.collect_map(map.iter().map(|(key, value)| (key.as_str(), value)))
}

pub fn deserialize<'deserialization_lifetime, TDeserializer>(
    deserializer: TDeserializer,
) -> Result<Map, TDeserializer::Error>
where
    TDeserializer: Deserializer<'deserialization_lifetime>,
{
    let entries = BTreeMap::<String, Dynamic>::deserialize(deserializer)?;

    Ok(entries
        .into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use indoc::indoc;
    use rhai::Dynamic;
    use rhai::Map;
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::json;

    #[derive(Deserialize, Serialize)]
    struct PropsHolder {
        #[serde(with = "super")]
        props: Map,
    }

    #[test]
    fn deserializes_nested_table_into_rhai_map() -> Result<()> {
        let PropsHolder { props } = toml::from_str(indoc! {r#"
            [props]
            title = "Hello"
            count = 3

            [props.nested]
            enabled = true
        "#})?;

        assert_eq!(
            props.get("title").map(|title| title.to_string()),
            Some("Hello".to_string())
        );
        assert_eq!(
            props.get("count").and_then(|count| count.as_int().ok()),
            Some(3)
        );
        assert_eq!(
            props
                .get("nested")
                .and_then(|nested| nested.clone().try_cast::<Map>())
                .and_then(|nested| nested.get("enabled").cloned())
                .and_then(|enabled| enabled.as_bool().ok()),
            Some(true)
        );

        Ok(())
    }

    #[test]
    fn rejects_non_table_value() {
        assert!(toml::from_str::<PropsHolder>("props = 5").is_err());
    }

    #[test]
    fn serializes_map_with_string_keys() -> Result<()> {
        let mut props = Map::new();

        props.insert("count".into(), Dynamic::from_int(3));
        props.insert("title".into(), Dynamic::from("Hello".to_string()));

        assert_eq!(
            serde_json::to_value(PropsHolder { props })?,
            json!({ "props": { "count": 3, "title": "Hello" } })
        );

        Ok(())
    }
}
