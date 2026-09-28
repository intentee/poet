use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::render_template_script::render_template_script;

#[test]
fn passes_expression_attribute_to_component() {
    assert!(
        render_template_script(
            r#"fn template(context) { component { <Note type={"warn"}>hi</Note> } }"#,
            DummyContext::default(),
        )
        .is_ok_and(|rendered| rendered.contains("note note--warn") && rendered.contains("hi"))
    );
}
