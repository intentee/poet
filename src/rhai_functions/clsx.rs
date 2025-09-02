use rhai::EvalAltResult;
use rhai::Map;

pub fn clsx(message: Map) -> Result<String, Box<EvalAltResult>> {
    let mut glued_class = String::new();

    for (key, value) in &message {
        if !value.is_bool() {
            return Err(format!("Expected only boolean map values, got: {value}").into());
        }

        let value_bool = value.as_bool()?;

        if value_bool {
            glued_class.push_str(&format!(" {key}"));
        }
    }

    Ok(glued_class.trim().to_string())
}
