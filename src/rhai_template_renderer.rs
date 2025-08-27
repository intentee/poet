use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Scope;

use crate::rhai_component_context::RhaiComponentContext;

pub type ShortcodeRenderer = dyn Fn(RhaiComponentContext, Dynamic, Dynamic) -> Result<String>;

pub struct RhaiTemplateRenderer {
    expression_engine: Engine,
    templates: DashMap<String, Box<ShortcodeRenderer>>,
}

impl RhaiTemplateRenderer {
    pub fn new(
        expression_engine: Engine,
        templates: DashMap<String, Box<ShortcodeRenderer>>,
    ) -> Self {
        Self {
            expression_engine,
            templates,
        }
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

    pub fn render_expression(
        &self,
        context: RhaiComponentContext,
        expression: &str,
    ) -> Result<Dynamic> {
        let mut scope = Scope::new();

        scope.push("context", context);

        Ok(self
            .expression_engine
            .eval_expression_with_scope(&mut scope, expression)?)
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
