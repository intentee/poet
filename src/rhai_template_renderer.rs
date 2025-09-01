use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Scope;

use crate::component_context::ComponentContext;

pub type ShortcodeRenderer = dyn Fn(ComponentContext, Dynamic, Dynamic) -> Result<String>;

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
        context: ComponentContext,
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
        context: ComponentContext,
        expression: &str,
    ) -> Result<Dynamic> {
        let mut scope = Scope::new();

        scope.push("context", context);

        Ok(self
            .expression_engine
            .eval_expression_with_scope(&mut scope, expression)?)
    }
}
