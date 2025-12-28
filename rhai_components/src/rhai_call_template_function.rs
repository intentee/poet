use anyhow::Result;
use rhai::AST;
use rhai::Engine;
use rhai::FuncArgs;
use rhai::ImmutableString;
use rhai::Position;
use rhai::Scope;

use crate::SmartStringLazy;

pub fn rhai_call_template_function(
    engine: &Engine,
    component_name: &str,
    args: impl FuncArgs,
) -> Result<SmartStringLazy> {
    let module = engine
        .module_resolver()
        .resolve(engine, None, component_name, Position::NONE)?;

    let tmp_ast = AST::new([], module);

    let result =
        engine.call_fn::<ImmutableString>(&mut Scope::new(), &tmp_ast, "template", args)?;

    Ok(result.into())
}
