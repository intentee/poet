use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Position;

pub fn error(message: Dynamic) -> Result<String, Box<EvalAltResult>> {
    Err(Box::new(EvalAltResult::ErrorRuntime(
        message,
        Position::NONE,
    )))
}

#[cfg(test)]
mod tests {
    use rhai::Dynamic;
    use rhai::EvalAltResult;
    use rhai::ImmutableString;

    use super::error;

    #[test]
    fn raises_runtime_error_carrying_the_message() {
        assert!(error(Dynamic::from("boom")).is_err_and(|eval_alt_result| {
            matches!(
                *eval_alt_result,
                EvalAltResult::ErrorRuntime(ref message, _)
                    if message.read_lock::<ImmutableString>().is_some_and(|text| *text == "boom")
            )
        }));
    }
}
