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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use rhai::CustomType;
    use rhai::Dynamic;
    use rhai::Engine;
    use rhai::Map;
    use rhai::TypeBuilder;
    use rhai::module_resolvers::FileModuleResolver;

    use super::ComponentReference;
    use super::RhaiTemplateRenderer;
    use super::RhaiTemplateRendererParams;
    use crate::builds_engine::BuildsEngine;
    use crate::component_syntax::component_registry::ComponentRegistry;

    fn fixtures_path() -> String {
        format!("{}/src/component_syntax/fixtures", env!("CARGO_MANIFEST_DIR"))
    }

    #[derive(Clone)]
    struct DummyContext;

    impl CustomType for DummyContext {
        fn build(_builder: TypeBuilder<Self>) {}
    }

    struct LocalBuilder {
        registry: Arc<ComponentRegistry>,
    }

    impl BuildsEngine for LocalBuilder {
        fn component_registry(&self) -> Arc<ComponentRegistry> {
            self.registry.clone()
        }

        fn prepare_engine(&self, engine: &mut Engine) -> Result<()> {
            engine.set_module_resolver(FileModuleResolver::new_with_path(fixtures_path()));
            engine.build_type::<DummyContext>();

            Ok(())
        }
    }

    fn registry_with(names: &[&str]) -> Arc<ComponentRegistry> {
        let registry = Arc::new(ComponentRegistry::default());

        for name in names {
            registry.register_component(ComponentReference {
                name: (*name).to_string(),
                path: (*name).to_string(),
            });
        }

        registry
    }

    fn build_renderer(names: &[&str]) -> Result<RhaiTemplateRenderer> {
        let builder = LocalBuilder {
            registry: registry_with(names),
        };

        builder.create_engine().and_then(|engine| {
            RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
                component_registry: builder.registry.clone(),
                expression_engine: engine,
            })
        })
    }

    #[test]
    fn build_succeeds_when_every_registered_component_resolves() -> Result<()> {
        assert!(build_renderer(&["Note"]).is_ok());

        Ok(())
    }

    #[test]
    fn build_returns_error_when_a_registered_component_cannot_be_resolved() -> Result<()> {
        assert!(build_renderer(&["NopeComponent"]).is_err());

        Ok(())
    }

    #[test]
    fn render_returns_template_not_found_error_for_unknown_component_name() -> Result<()> {
        assert!(build_renderer(&[]).is_ok_and(|renderer| {
            renderer
                .render("Unknown", DummyContext, Dynamic::UNIT, Dynamic::UNIT)
                .is_err_and(|error| error.to_string().contains("Template 'Unknown' not found"))
        }));

        Ok(())
    }

    #[test]
    fn render_invokes_template_function_for_known_component() -> Result<()> {
        let mut props = Map::new();

        props.insert("type".into(), "warn".into());

        assert!(build_renderer(&["Note"]).is_ok_and(|renderer| {
            renderer
                .render(
                    "Note",
                    DummyContext,
                    Dynamic::from_map(props),
                    Dynamic::from(String::new()),
                )
                .is_ok_and(|rendered| rendered.contains("note--warn"))
        }));

        Ok(())
    }

    #[test]
    fn render_expression_evaluates_simple_expression() -> Result<()> {
        assert!(build_renderer(&[]).is_ok_and(|renderer| {
            renderer
                .render_expression(DummyContext, "40 + 2")
                .is_ok_and(|value| value.as_int().is_ok_and(|as_int| as_int == 42))
        }));

        Ok(())
    }

    #[test]
    fn render_expression_wraps_evaluation_failure_with_context_message() -> Result<()> {
        assert!(build_renderer(&[]).is_ok_and(|renderer| {
            renderer
                .render_expression(DummyContext, "not valid @ rhai!")
                .is_err_and(|error| {
                    error.to_string().contains("Expression failed: 'not valid @ rhai!'")
                })
        }));

        Ok(())
    }
}
