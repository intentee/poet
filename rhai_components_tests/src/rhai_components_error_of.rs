use rhai::EvalAltResult;
use rhai_components::rhai_components_error::RhaiComponentsError;

#[must_use]
pub fn rhai_components_error_of(eval_alt_result: &EvalAltResult) -> Option<&RhaiComponentsError> {
    match eval_alt_result {
        EvalAltResult::ErrorSystem(_, source) => source.downcast_ref::<RhaiComponentsError>(),
        _ => None,
    }
}
