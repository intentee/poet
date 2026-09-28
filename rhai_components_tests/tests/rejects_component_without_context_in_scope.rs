use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::fixtures_engine::fixtures_engine;
use rhai_components_tests::rhai_components_error_of::rhai_components_error_of;

#[test]
fn rejects_component_without_context_in_scope() {
    assert!(
        fixtures_engine()
            .eval::<String>("component { <Note /> }")
            .is_err_and(|eval_alt_result| matches!(
                rhai_components_error_of(&eval_alt_result),
                Some(RhaiComponentsError::ComponentContextNotInScope { variable_name })
                    if variable_name == "context"
            ))
    );
}
