use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::render_template_script::render_template_script;

#[test]
fn passes_empty_attribute_to_component_as_true() {
    assert!(
        render_template_script(
            "fn template(context) { component { <Bare disabled>hi</Bare> } }",
            DummyContext::default(),
        )
        .is_ok_and(|rendered| rendered.contains(r#"<span data-disabled="yes">hi</span>"#))
    );
}
