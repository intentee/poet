use std::error::Error;

use poet_prompt::prompt_error::PromptError;
use rhai::EvalAltResult;
use rhai_components::rhai_components_error::RhaiComponentsError;

#[must_use]
pub fn expression_system_error_of<TError: Error + 'static>(
    prompt_error: &PromptError,
) -> Option<&TError> {
    match prompt_error {
        PromptError::EvaluateExpression {
            source: RhaiComponentsError::EvaluateExpression { source, .. },
            ..
        } => match source.as_ref() {
            EvalAltResult::ErrorSystem(_, system_error) => system_error.downcast_ref::<TError>(),
            _ => None,
        },
        _ => None,
    }
}
