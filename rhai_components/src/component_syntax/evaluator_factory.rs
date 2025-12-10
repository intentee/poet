use std::sync::Arc;

use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;
use rhai::Position;

use super::component_registry::ComponentRegistry;
use super::eval_tag_stack_node::eval_tag_stack_node;
use super::expression_collection::ExpressionCollection;
use super::tag_stack_node::TagStackNode;

pub struct EvaluatorFactory {
    pub component_registry: Arc<ComponentRegistry>,
}

impl EvaluatorFactory {
    pub fn create_component_evaluator(
        &self,
    ) -> impl Fn(&mut EvalContext, &[Expression], &Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
    + Send
    + Sync
    + 'static {
        let component_registry_clone = self.component_registry.clone();

        move |eval_context: &mut EvalContext, inputs: &[Expression], state: &Dynamic| {
            let mut expression_collection = ExpressionCollection {
                expressions: inputs.to_vec(),
            };

            let rendered_tag_stack = eval_tag_stack_node(
                component_registry_clone.clone(),
                eval_context,
                &state.clone().try_cast::<TagStackNode>().ok_or_else(|| {
                    EvalAltResult::ErrorRuntime(
                        "Expected TagStackNode in tag state".into(),
                        Position::NONE,
                    )
                })?,
                &mut expression_collection,
            )?;

            Ok(Dynamic::from(rendered_tag_stack.to_string()))
        }
    }
}
