use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::fixtures_renderer::fixtures_renderer;

#[test]
fn renderer_evaluates_expression() -> Result<(), RhaiComponentsError> {
    assert!(
        fixtures_renderer(&[])?
            .render_expression(DummyContext::default(), "40 + 2")
            .is_ok_and(|value| value.as_int() == Ok(42))
    );

    Ok(())
}
