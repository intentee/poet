use rhai::EvalAltResult;
use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::render_template_script::render_template_script;
use rhai_components_tests::root_cause::root_cause;

#[test]
fn propagates_component_children_failure() {
    assert!(
        render_template_script(
            "fn template(context) { component { <Note>{missing_variable}</Note> } }",
            DummyContext::default(),
        )
        .is_err_and(|eval_alt_result| matches!(
            root_cause(&eval_alt_result),
            EvalAltResult::ErrorVariableNotFound(variable_name, _)
                if variable_name == "missing_variable"
        ))
    );
}
