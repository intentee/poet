use std::sync::Arc;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Position;
use rhai::Scope;

use crate::component_syntax::component_reference::ComponentReference;
use crate::rhai_call_template_function::rhai_call_template_function;
use crate::rhai_template_renderer_params::RhaiTemplateRendererParams;

#[derive(Clone)]
pub struct RhaiTemplateRenderer {
    expression_engine: Arc<Engine>,
    templates: Arc<DashMap<String, ComponentReference>>,
}

impl RhaiTemplateRenderer {
    pub fn build(
        RhaiTemplateRendererParams {
            component_registry,
            mut expression_engine,
        }: RhaiTemplateRendererParams,
    ) -> Result<Self> {
        let templates: DashMap<String, ComponentReference> = DashMap::new();

        for entry in &component_registry.components {
            let component_reference = entry.value();

            let module_resolver = expression_engine.module_resolver();
            let module = module_resolver.resolve(
                &expression_engine,
                None,
                &component_reference.name,
                Position::NONE,
            )?;

            expression_engine.register_static_module(component_reference.name.clone(), module);

            templates.insert(
                component_reference.name.clone(),
                component_reference.clone(),
            );
        }

        Ok(Self {
            expression_engine: expression_engine.into(),
            templates: templates.into(),
        })
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
            rhai_call_template_function(
                &self.expression_engine,
                &component_reference.name,
                (context, props, content),
            )
            .map(Into::into)
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
            .eval_with_scope(&mut scope, expression)
            .context(format!("Expression failed: '{expression}'"))
    }
}
