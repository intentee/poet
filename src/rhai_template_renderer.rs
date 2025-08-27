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
        props: Dynamic,
        content: Dynamic,
    ) -> Result<String> {
        if let Some(renderer) = self.templates.get(name) {
            renderer(context, props, content)
        } else {
            Err(anyhow!("Template '{}' not found", name))
        }
    }

    pub fn render_without_props(
        &self,
        name: &str,
        context: RhaiComponentContext,
        content: String,
    ) -> Result<String> {
        self.render(
            name,
            context,
            Dynamic::from_map(rhai::Map::new()),
            Dynamic::from(content),
        )
    }
}
