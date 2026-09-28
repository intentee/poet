use rhai_components_tests::fixtures_engine::fixtures_engine;

#[test]
fn renders_self_closing_tag_after_attribute_name() {
    assert!(
        fixtures_engine()
            .eval::<String>("component { <input checked/> }")
            .is_ok_and(|rendered| rendered.contains("<input checked>"))
    );
}
