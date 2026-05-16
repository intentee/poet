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

fn cast_state_to_tag_stack_node(state: &Dynamic) -> Result<TagStackNode, Box<EvalAltResult>> {
    state.clone().try_cast::<TagStackNode>().ok_or_else(|| {
        Box::new(EvalAltResult::ErrorRuntime(
            "Expected TagStackNode in tag state".into(),
            Position::NONE,
        ))
    })
}

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

            let tag_stack_node = cast_state_to_tag_stack_node(state)?;

            let rendered_tag_stack = eval_tag_stack_node(
                component_registry_clone.clone(),
                eval_context,
                &tag_stack_node,
                &mut expression_collection,
            )?;

            Ok(Dynamic::from(rendered_tag_stack.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;
    use std::sync::Arc;

    use anyhow::Result;
    use rhai::Dynamic;
    use rhai::Engine;
    use rhai::EvalAltResult;

    use super::ComponentRegistry;
    use super::EvaluatorFactory;
    use super::Position;
    use super::cast_state_to_tag_stack_node;

    #[test]
    fn cast_state_to_tag_stack_node_returns_runtime_error_for_non_tag_stack_node_dynamic()
    -> Result<()> {
        let state = Dynamic::from(42_i64);
        let reference = discriminant(&EvalAltResult::ErrorRuntime(Dynamic::UNIT, Position::NONE));

        assert!(cast_state_to_tag_stack_node(&state).is_err_and(|boxed| {
            discriminant(boxed.as_ref()) == reference
                && boxed.to_string().contains("Expected TagStackNode in tag state")
        }));

        Ok(())
    }

    #[test]
    fn evaluator_closure_returns_error_when_state_is_not_a_tag_stack_node() -> Result<()> {
        let factory = EvaluatorFactory {
            component_registry: Arc::new(ComponentRegistry::default()),
        };
        let mut engine = Engine::new();

        engine.register_custom_syntax_without_look_ahead_raw(
            "bad_syntax",
            |_symbols, _state| Ok(None),
            false,
            factory.create_component_evaluator(),
        );

        assert!(engine
            .eval::<String>("bad_syntax")
            .is_err_and(|error| error.to_string().contains("Expected TagStackNode")));

        Ok(())
    }
}
