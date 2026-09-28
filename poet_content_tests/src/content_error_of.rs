use poet_content::content_error::ContentError;
use rhai::EvalAltResult;

#[must_use]
pub fn content_error_of(eval_alt_result: &EvalAltResult) -> Option<&ContentError> {
    match eval_alt_result {
        EvalAltResult::ErrorSystem(_, source) => source.downcast_ref::<ContentError>(),
        _ => None,
    }
}
