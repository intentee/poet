use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::fixtures_renderer::fixtures_renderer;

#[test]
fn renderer_reports_failed_expression() -> Result<(), RhaiComponentsError> {
    assert!(matches!(
        fixtures_renderer(&[])?.render_expression(DummyContext::default(), "not valid @ rhai!"),
        Err(RhaiComponentsError::EvaluateExpression { expression, .. })
            if expression == "not valid @ rhai!"
    ));

    Ok(())
}
