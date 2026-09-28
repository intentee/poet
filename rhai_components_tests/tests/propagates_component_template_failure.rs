use rhai::EvalAltResult;
use rhai::ImmutableString;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::render_template_script::render_template_script;
use rhai_components_tests::rhai_components_error_of::rhai_components_error_of;
use rhai_components_tests::root_cause::root_cause;

#[test]
fn propagates_component_template_failure() {
    assert!(
        render_template_script(
            "fn template(context) { component { <ThrowingComponent /> } }",
            DummyContext::default(),
        )
        .is_err_and(|eval_alt_result| {
            matches!(
                rhai_components_error_of(&eval_alt_result),
                Some(RhaiComponentsError::CallTemplateFunction { component_name, .. })
                    if component_name == "ThrowingComponent"
            ) && matches!(
                root_cause(&eval_alt_result),
                EvalAltResult::ErrorRuntime(message, _)
                    if message
                        .read_lock::<ImmutableString>()
                        .is_some_and(|text| *text == "boom from ThrowingComponent")
            )
        })
    );
}
