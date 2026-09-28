use rhai::Engine;
use rhai::module_resolvers::FileModuleResolver;
use rhai_components::component_syntax::evaluate_component::evaluate_component;
use rhai_components::component_syntax::parse_component::parse_component;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::rhai_components_error_of::rhai_components_error_of;

#[test]
fn rejects_evaluation_without_component_nesting_depth() {
    let mut engine = Engine::new();

    engine.set_module_resolver(FileModuleResolver::new_with_path(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures"
    )));
    engine.register_custom_syntax_without_look_ahead_raw(
        "component",
        parse_component,
        false,
        evaluate_component,
    );

    assert!(
        engine
            .eval::<String>("let context = (); component { <Note /> }")
            .is_err_and(|eval_alt_result| matches!(
                rhai_components_error_of(&eval_alt_result),
                Some(RhaiComponentsError::MissingComponentNestingDepth)
            ))
    );
}
