use rhai::Dynamic;
use rhai::EvalAltResult;

pub fn has(value: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    match value.type_name() {
        "()" => Ok(false),
        "array" => Ok(value.as_array_ref()?.len() > 0),
        "bool" => Ok(value.as_bool()?),
        "map" => Ok(value.as_map_ref()?.len() > 0),
        "string" => Ok(!value.into_string()?.is_empty()),
        _ => Ok(true),
    }
}
