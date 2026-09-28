use rhai::Array;
use rhai::Dynamic;
use rhai::ImmutableString;
use rhai::Map;

#[must_use]
pub fn has(value: Dynamic) -> bool {
    if value.is_unit() {
        return false;
    }

    if let Some(array) = value.read_lock::<Array>() {
        return !array.is_empty();
    }

    if let Some(flag) = value.read_lock::<bool>() {
        return *flag;
    }

    if let Some(map) = value.read_lock::<Map>() {
        return !map.is_empty();
    }

    if let Some(string) = value.read_lock::<ImmutableString>() {
        return !string.is_empty();
    }

    true
}

#[cfg(test)]
mod tests {
    use rhai::Array;
    use rhai::Dynamic;
    use rhai::Map;

    use super::has;

    #[test]
    fn unit_is_absent() {
        assert!(!has(Dynamic::UNIT));
    }

    #[test]
    fn array_is_present_when_not_empty() {
        assert!(!has(Dynamic::from(Array::new())));
        assert!(has(Dynamic::from(vec![Dynamic::from(1_i64)])));
    }

    #[test]
    fn bool_is_present_when_true() {
        assert!(has(Dynamic::from(true)));
        assert!(!has(Dynamic::from(false)));
    }

    #[test]
    fn map_is_present_when_not_empty() {
        let mut non_empty_map = Map::new();

        non_empty_map.insert("key".into(), Dynamic::from(1_i64));

        assert!(!has(Dynamic::from_map(Map::new())));
        assert!(has(Dynamic::from_map(non_empty_map)));
    }

    #[test]
    fn string_is_present_when_not_empty() {
        assert!(!has(Dynamic::from(String::new())));
        assert!(has(Dynamic::from("text".to_owned())));
    }

    #[test]
    fn other_values_are_present() {
        assert!(has(Dynamic::from(42_i64)));
    }
}
