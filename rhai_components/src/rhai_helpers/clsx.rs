use rhai::EvalAltResult;
use rhai::Map;

pub fn clsx(message: Map) -> Result<String, Box<EvalAltResult>> {
    let mut glued_class = String::new();

    for (key, value) in &message {
        if !value.is_bool() {
            return Err(format!("Expected only boolean map values, got: {value}").into());
        }

        if value.as_bool().unwrap_or(false) {
            glued_class.push_str(&format!(" {key}"));
        }
    }

    Ok(glued_class.trim().to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rhai::Dynamic;
    use rhai::Map;

    use super::clsx;

    fn make_map(entries: &[(&str, Dynamic)]) -> Map {
        let mut map = Map::new();

        for (key, value) in entries {
            map.insert((*key).into(), value.clone());
        }

        map
    }

    #[test]
    fn joins_truthy_keys_with_single_space_and_drops_falsy_keys() -> Result<()> {
        let map = make_map(&[
            ("a", Dynamic::from(true)),
            ("b", Dynamic::from(false)),
            ("c", Dynamic::from(true)),
        ]);

        assert!(clsx(map).is_ok_and(|joined| joined == "a c"));

        Ok(())
    }

    #[test]
    fn returns_empty_string_for_empty_map() -> Result<()> {
        assert!(clsx(Map::new()).is_ok_and(|joined| joined.is_empty()));

        Ok(())
    }

    #[test]
    fn returns_error_when_value_is_not_bool() -> Result<()> {
        let map = make_map(&[("a", Dynamic::from(1_i64))]);

        assert!(clsx(map).is_err_and(|error| {
            error.to_string().contains("Expected only boolean map values")
        }));

        Ok(())
    }
}
