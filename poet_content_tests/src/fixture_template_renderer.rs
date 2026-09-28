use std::sync::Arc;

use poet_content::register_content_rhai_types::register_content_rhai_types;
use rhai_components::component_syntax::component_registry::ComponentRegistry;
use rhai_components::create_component_engine::create_component_engine;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;

pub fn fixture_template_renderer() -> Result<RhaiTemplateRenderer, RhaiComponentsError> {
    let mut expression_engine = create_component_engine();

    register_content_rhai_types(&mut expression_engine);

    RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
        component_registry: Arc::new(ComponentRegistry::default()),
        expression_engine,
    })
}
