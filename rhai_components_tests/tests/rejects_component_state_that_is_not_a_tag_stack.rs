use rhai_components::component_syntax::evaluate_component::evaluate_component;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::fixtures_engine::fixtures_engine;
use rhai_components_tests::rhai_components_error_of::rhai_components_error_of;

#[test]
fn rejects_component_state_that_is_not_a_tag_stack() {
    let mut engine = fixtures_engine();

    engine.register_custom_syntax_without_look_ahead_raw(
        "broken_component",
        |_symbols, _state| Ok(None),
        false,
        evaluate_component,
    );

    assert!(
        engine
            .eval::<String>("broken_component")
            .is_err_and(|eval_alt_result| matches!(
                rhai_components_error_of(&eval_alt_result),
                Some(RhaiComponentsError::UnexpectedComponentState)
            ))
    );
}
