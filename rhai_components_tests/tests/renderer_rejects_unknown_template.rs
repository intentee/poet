use rhai::Dynamic;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::fixtures_renderer::fixtures_renderer;

#[test]
fn renderer_rejects_unknown_template() -> Result<(), RhaiComponentsError> {
    assert!(matches!(
        fixtures_renderer(&[])?.render(
            "Unknown",
            DummyContext::default(),
            Dynamic::UNIT,
            Dynamic::UNIT
        ),
        Err(RhaiComponentsError::TemplateNotFound { name }) if name == "Unknown"
    ));

    Ok(())
}
