use rhai::Dynamic;
use rhai_components::component_syntax::evaluate_component::evaluate_component;
use rhai_components::component_syntax::expression_reference::ExpressionReference;
use rhai_components::component_syntax::tag_stack::TagStack;
use rhai_components::component_syntax::tag_stack_node::TagStackNode;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::fixtures_engine::fixtures_engine;
use rhai_components_tests::rhai_components_error_of::rhai_components_error_of;

#[test]
fn rejects_tag_stack_referencing_missing_expression() {
    let mut engine = fixtures_engine();

    engine.register_custom_syntax_without_look_ahead_raw(
        "dangling_expression",
        |_symbols, state| {
            *state = Dynamic::from(TagStack {
                children: vec![TagStackNode::BodyExpression(ExpressionReference {
                    expression_index: 0,
                })],
            });

            Ok(None)
        },
        false,
        evaluate_component,
    );

    assert!(
        engine
            .eval::<String>("dangling_expression")
            .is_err_and(|eval_alt_result| matches!(
                rhai_components_error_of(&eval_alt_result),
                Some(RhaiComponentsError::ExpressionIndexOutOfBounds {
                    expression_index: 0
                })
            ))
    );
}
