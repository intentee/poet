use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::render_template_script::render_template_script;

#[test]
fn concatenates_array_body_expression() {
    assert!(
        render_template_script(
            r#"fn template(context) { component { <div>{["a", "b", "c"]}</div> } }"#,
            DummyContext::default(),
        )
        .is_ok_and(|rendered| rendered.contains("<div>abc</div>"))
    );
}
