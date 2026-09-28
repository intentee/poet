use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;

use super::eval_tag_stack::eval_tag_stack;
use super::expression_collection::ExpressionCollection;
use super::tag_stack::TagStack;
use crate::rhai_components_error::RhaiComponentsError;

pub fn evaluate_component(
    eval_context: &mut EvalContext,
    inputs: &[Expression],
    state: &Dynamic,
) -> Result<Dynamic, Box<EvalAltResult>> {
    let tag_stack = state
        .read_lock::<TagStack>()
        .ok_or(RhaiComponentsError::UnexpectedComponentState)?;

    Ok(Dynamic::from(eval_tag_stack(
        eval_context,
        &ExpressionCollection {
            expressions: inputs,
        },
        &tag_stack,
    )?))
}
