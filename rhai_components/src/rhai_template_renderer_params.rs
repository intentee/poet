use std::sync::Arc;

use rhai::Engine;

use crate::component_syntax::component_registry::ComponentRegistry;

pub struct RhaiTemplateRendererParams {
    pub component_registry: Arc<ComponentRegistry>,
    pub expression_engine: Engine,
}
