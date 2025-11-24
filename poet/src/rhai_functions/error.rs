use rhai::Dynamic;
use rhai::EvalAltResult;

pub fn error(message: Dynamic) -> Result<String, Box<EvalAltResult>> {
    Err(message.to_string().into())
}
