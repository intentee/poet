use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components_tests::fixtures_renderer::fixtures_renderer;

#[test]
fn renderer_rejects_unresolvable_component() {
    assert!(matches!(
        fixtures_renderer(&["MissingComponent"]),
        Err(RhaiComponentsError::ResolveComponentModule { component_name, .. })
            if component_name == "MissingComponent"
    ));
}
