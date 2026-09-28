use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai_components::component_nesting_depth::ComponentNestingDepth;
use rhai_components::rhai_call_template_function::rhai_call_template_function;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::fixtures_engine::fixtures_engine;

#[test]
fn call_template_function_rejects_module_without_template() {
    assert!(matches!(
        rhai_call_template_function(
            &fixtures_engine(),
            "NoTemplate",
            ComponentNestingDepth::default(),
            (Dynamic::UNIT, Dynamic::UNIT, Dynamic::UNIT),
        ),
        Err(RhaiComponentsError::CallTemplateFunction { component_name, source })
            if component_name == "NoTemplate"
                && matches!(*source, EvalAltResult::ErrorFunctionNotFound(..))
    ));
}
