use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use rhai::Dynamic;

use crate::rhai_component_context::RhaiComponentContext;

pub type ShortcodeRenderer = dyn Fn(RhaiComponentContext, Dynamic, Dynamic) -> Result<String>;

pub struct RhaiTemplateRenderer {
    templates: DashMap<String, Box<ShortcodeRenderer>>,
}

impl RhaiTemplateRenderer {
    pub fn new(templates: DashMap<String, Box<ShortcodeRenderer>>) -> Self {
        Self { templates }
    }

    pub fn render(
        &self,
        name: &str,
        context: RhaiComponentContext,
        params: Dynamic,
        content: Dynamic,
    ) -> Result<String> {
        if let Some(renderer) = self.templates.get(name) {
            renderer(context, params, content)
        } else {
            Err(anyhow!("Template '{}' not found", name))
        }
    }
}
