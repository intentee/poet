use rhai::EvalAltResult;
use rhai::ImmutableString;
use rhai::Map;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

pub fn clsx(message: Map) -> Result<ImmutableString, Box<EvalAltResult>> {
    let mut glued_class = SmartString::new_const();

    for (key, value) in &message {
        if !value.is_bool() {
            return Err(format!("Expected only boolean map values, got: {value}").into());
        }

        let value_bool = value.as_bool()?;

        if value_bool {
            glued_class.push_str(&format!(" {key}"));
        }
    }

    let trimmed = glued_class.trim();

    if trimmed.len() == glued_class.len() {
        Ok(glued_class.into())
    } else {
        Ok(trimmed.into())
    }
}
