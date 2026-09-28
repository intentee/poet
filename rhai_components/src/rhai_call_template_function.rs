use rhai::AST;
use rhai::CallFnOptions;
use rhai::Engine;
use rhai::FuncArgs;
use rhai::Position;
use rhai::Scope;

use crate::component_nesting_depth::ComponentNestingDepth;
use crate::rhai_components_error::RhaiComponentsError;

pub fn rhai_call_template_function(
    engine: &Engine,
    component_name: &str,
    component_nesting_depth: ComponentNestingDepth,
    template_arguments: impl FuncArgs,
) -> Result<String, RhaiComponentsError> {
    let module = engine
        .module_resolver()
        .resolve(engine, None, component_name, Position::NONE)
        .map_err(|source| RhaiComponentsError::ResolveComponentModule {
            component_name: component_name.to_owned(),
            source,
        })?;

    engine
        .call_fn_with_options(
            CallFnOptions::new().with_tag(component_nesting_depth),
            &mut Scope::new(),
            &AST::new([], module),
            "template",
            template_arguments,
        )
        .map_err(|source| RhaiComponentsError::CallTemplateFunction {
            component_name: component_name.to_owned(),
            source,
        })
}
