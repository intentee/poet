use std::sync::Arc;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Scope;

use crate::rhai_components::component_reference::ComponentReference;

#[derive(Clone)]
pub struct RhaiTemplateRenderer {
    expression_engine: Arc<Engine>,
    templates: Arc<DashMap<String, ComponentReference>>,
}

impl RhaiTemplateRenderer {
    pub fn new(
        expression_engine: Arc<Engine>,
        templates: Arc<DashMap<String, ComponentReference>>,
    ) -> Self {
        Self {
            expression_engine,
            templates,
        }
    }

    pub fn render<TComponentContext>(
        &self,
        name: &str,
        context: TComponentContext,
        props: Dynamic,
        content: Dynamic,
    ) -> Result<String>
    where
        TComponentContext: CustomType,
    {
        if let Some(component_reference) = self.templates.get(name) {
            Ok(self.expression_engine.eval_fn_call(
                component_reference.global_fn_name.clone(),
                None,
                (context, props, content),
            )?)
        } else {
            Err(anyhow!("Template '{name}' not found"))
        }
    }

    pub fn render_expression<TComponentContext>(
        &self,
        context: TComponentContext,
        expression: &str,
    ) -> Result<Dynamic>
    where
        TComponentContext: CustomType,
    {
        let mut scope = Scope::new();

        scope.push("context", context);

        self.expression_engine
            .eval_expression_with_scope(&mut scope, expression)
            .context(format!("Expression failed: '{expression}'"))
    }
}
