use rhai_components_tests::fixtures_engine::fixtures_engine;

#[test]
fn component_engine_registers_template_helpers() {
    assert!(
        fixtures_engine()
            .eval::<String>(r"clsx(#{ enabled: true, disabled: false })")
            .is_ok_and(|glued_class| glued_class == "enabled")
    );
}
