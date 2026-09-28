use std::sync::Arc;

use rhai::CustomType;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Position;
use rhai::Scope;

use crate::component_context_variable_name::COMPONENT_CONTEXT_VARIABLE_NAME;
use crate::component_nesting_depth::ComponentNestingDepth;
use crate::component_syntax::component_registry::ComponentRegistry;
use crate::rhai_call_template_function::rhai_call_template_function;
use crate::rhai_components_error::RhaiComponentsError;
use crate::rhai_template_renderer_params::RhaiTemplateRendererParams;

#[derive(Clone)]
pub struct RhaiTemplateRenderer {
    component_registry: Arc<ComponentRegistry>,
    expression_engine: Arc<Engine>,
}

impl RhaiTemplateRenderer {
    pub fn build(
        RhaiTemplateRendererParams {
            component_registry,
            mut expression_engine,
        }: RhaiTemplateRendererParams,
    ) -> Result<Self, RhaiComponentsError> {
        for component_reference in component_registry.components.iter() {
            let module = expression_engine
                .module_resolver()
                .resolve(
                    &expression_engine,
                    None,
                    &component_reference.name,
                    Position::NONE,
                )
                .map_err(|source| RhaiComponentsError::ResolveComponentModule {
                    component_name: component_reference.name.clone(),
                    source,
                })?;

            expression_engine.register_static_module(component_reference.name.clone(), module);
        }

        Ok(Self {
            component_registry,
            expression_engine: Arc::new(expression_engine),
        })
    }

    pub fn render<TComponentContext>(
        &self,
        name: &str,
        component_context: TComponentContext,
        props: Dynamic,
        content: Dynamic,
    ) -> Result<String, RhaiComponentsError>
    where
        TComponentContext: CustomType,
    {
        if !self.component_registry.components.contains(name) {
            return Err(RhaiComponentsError::TemplateNotFound {
                name: name.to_owned(),
            });
        }

        rhai_call_template_function(
            &self.expression_engine,
            name,
            ComponentNestingDepth::default(),
            (component_context, props, content),
        )
    }

    pub fn render_expression<TComponentContext>(
        &self,
        context: TComponentContext,
        expression: &str,
    ) -> Result<Dynamic, RhaiComponentsError>
    where
        TComponentContext: CustomType,
    {
        let mut scope = Scope::new();

        scope.push(COMPONENT_CONTEXT_VARIABLE_NAME, context);

        self.expression_engine
            .eval_with_scope(&mut scope, expression)
            .map_err(|source| RhaiComponentsError::EvaluateExpression {
                expression: expression.to_owned(),
                source,
            })
    }
}
