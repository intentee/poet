use rhai::Dynamic;
use rhai::Map;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::dummy_context::DummyContext;
use rhai_components_tests::fixtures_renderer::fixtures_renderer;

#[test]
fn renderer_renders_registered_component() -> Result<(), RhaiComponentsError> {
    let mut props = Map::new();

    props.insert("type".into(), "warn".into());

    assert!(
        fixtures_renderer(&["Note"])?
            .render(
                "Note",
                DummyContext::default(),
                Dynamic::from_map(props),
                Dynamic::from(String::new()),
            )
            .is_ok_and(|rendered| rendered.contains("note note--warn"))
    );

    Ok(())
}
