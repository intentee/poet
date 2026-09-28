use rhai::EvalAltResult;
use rhai_components::rhai_components_error::RhaiComponentsError;

use crate::rhai_components_error_of::rhai_components_error_of;

pub fn root_cause(eval_alt_result: &EvalAltResult) -> &EvalAltResult {
    match eval_alt_result {
        EvalAltResult::ErrorInFunctionCall(_, _, inner_eval_alt_result, _) => {
            root_cause(inner_eval_alt_result)
        }
        _ => match rhai_components_error_of(eval_alt_result) {
            Some(
                RhaiComponentsError::CallTemplateFunction { source, .. }
                | RhaiComponentsError::EvaluateExpression { source, .. }
                | RhaiComponentsError::ResolveComponentModule { source, .. },
            ) => root_cause(source),
            _ => eval_alt_result,
        },
    }
}
