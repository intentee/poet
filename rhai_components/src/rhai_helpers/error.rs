use rhai::Dynamic;
use rhai::EvalAltResult;

pub fn error(message: Dynamic) -> Result<String, Box<EvalAltResult>> {
    Err(message.to_string().into())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rhai::Dynamic;

    use super::error;

    #[test]
    fn always_returns_runtime_error_with_stringified_message() -> Result<()> {
        assert!(
            error(Dynamic::from("boom")).is_err_and(|boxed| { boxed.to_string().contains("boom") })
        );

        Ok(())
    }
}
