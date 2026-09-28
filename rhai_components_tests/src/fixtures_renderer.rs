use std::sync::Arc;

use rhai_components::component_syntax::component_reference::ComponentReference;
use rhai_components::component_syntax::component_registry::ComponentRegistry;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;

use crate::fixtures_engine::fixtures_engine;

pub fn fixtures_renderer(
    component_names: &[&str],
) -> Result<RhaiTemplateRenderer, RhaiComponentsError> {
    let component_registry = ComponentRegistry::default();

    for component_name in component_names {
        component_registry.register_component(ComponentReference {
            name: (*component_name).to_owned(),
        });
    }

    RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
        component_registry: Arc::new(component_registry),
        expression_engine: fixtures_engine(),
    })
}
