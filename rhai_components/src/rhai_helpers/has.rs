use rhai::Dynamic;
use rhai::EvalAltResult;

pub fn has(value: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    Ok(match value.type_name() {
        "()" => false,
        "array" => value
            .as_array_ref()
            .map(|array| !array.is_empty())
            .unwrap_or(false),
        "bool" => value.as_bool().unwrap_or(false),
        "map" => value
            .as_map_ref()
            .map(|map| !map.is_empty())
            .unwrap_or(false),
        "string" => value
            .into_string()
            .map(|string| !string.is_empty())
            .unwrap_or(false),
        _ => true,
    })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rhai::Array;
    use rhai::Dynamic;
    use rhai::Map;

    use super::has;

    #[test]
    fn has_returns_false_for_unit() -> Result<()> {
        assert!(has(Dynamic::UNIT).is_ok_and(|present| !present));

        Ok(())
    }

    #[test]
    fn has_returns_non_emptiness_for_array() -> Result<()> {
        let empty: Array = Vec::new();
        let non_empty: Array = vec![Dynamic::from(1_i64)];

        assert!(has(Dynamic::from(empty)).is_ok_and(|present| !present));
        assert!(has(Dynamic::from(non_empty)).is_ok_and(|present| present));

        Ok(())
    }

    #[test]
    fn has_returns_its_value_for_bool() -> Result<()> {
        assert!(has(Dynamic::from(true)).is_ok_and(|present| present));
        assert!(has(Dynamic::from(false)).is_ok_and(|present| !present));

        Ok(())
    }

    #[test]
    fn has_returns_non_emptiness_for_map() -> Result<()> {
        let empty = Map::new();
        let mut non_empty = Map::new();

        non_empty.insert("a".into(), Dynamic::from(1_i64));

        assert!(has(Dynamic::from_map(empty)).is_ok_and(|present| !present));
        assert!(has(Dynamic::from_map(non_empty)).is_ok_and(|present| present));

        Ok(())
    }

    #[test]
    fn has_returns_non_emptiness_for_string() -> Result<()> {
        assert!(has(Dynamic::from(String::new())).is_ok_and(|present| !present));
        assert!(has(Dynamic::from("x".to_string())).is_ok_and(|present| present));

        Ok(())
    }

    #[test]
    fn has_returns_true_for_other_types() -> Result<()> {
        assert!(has(Dynamic::from(42_i64)).is_ok_and(|present| present));

        Ok(())
    }
}
