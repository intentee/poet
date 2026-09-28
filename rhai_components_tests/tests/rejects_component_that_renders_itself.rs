use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::fixtures_engine::fixtures_engine;
use rhai_components_tests::rhai_components_error_of::rhai_components_error_of;
use rhai_components_tests::root_cause::root_cause;

#[test]
fn rejects_component_that_renders_itself() {
    assert!(
        fixtures_engine()
            .eval::<String>("let context = (); component { <Recursive /> }")
            .is_err_and(|eval_alt_result| matches!(
                rhai_components_error_of(root_cause(&eval_alt_result)),
                Some(RhaiComponentsError::ComponentNestingTooDeep {
                    component_name,
                    maximum_depth: 40,
                }) if component_name == "Recursive"
            ))
    );
}
