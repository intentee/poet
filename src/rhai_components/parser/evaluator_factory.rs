use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;
use rhai::Position;

use super::eval_tag_stack_node::eval_tag_stack_node;
use super::expression_collection::ExpressionCollection;
use super::tag_stack_node::TagStackNode;

pub struct EvaluatorFactory {}

impl EvaluatorFactory {
    pub fn create_component_evaluator_with_context<TComponentContext>(
        &self,
        component_context: TComponentContext,
    ) -> impl Fn(&mut EvalContext, &[Expression], &Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
    + Send
    + Sync
    + 'static
    where
        TComponentContext: Clone + Send + Sync + 'static,
    {
        move |eval_context: &mut EvalContext, inputs: &[Expression], state: &Dynamic| {
            let mut expression_collection = ExpressionCollection {
                expressions: inputs.to_vec(),
            };

            let rendered_tag_stack = eval_tag_stack_node(
                component_context.clone(),
                eval_context,
                &state.clone().try_cast::<TagStackNode>().ok_or_else(|| {
                    EvalAltResult::ErrorRuntime(
                        "Expected TagStackNode in tag state".into(),
                        Position::NONE,
                    )
                })?,
                &mut expression_collection,
            )?;

            Ok(Dynamic::from(rendered_tag_stack))
        }
    }
}
