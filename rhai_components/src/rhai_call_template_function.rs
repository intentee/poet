use anyhow::Result;
use rhai::AST;
use rhai::Engine;
use rhai::FuncArgs;
use rhai::Position;
use rhai::Scope;

pub fn rhai_call_template_function(
    engine: &Engine,
    component_name: &str,
    args: impl FuncArgs,
) -> Result<String> {
    let module = engine
        .module_resolver()
        .resolve(engine, None, component_name, Position::NONE)?;

    let tmp_ast = AST::new([], module);

    Ok(engine.call_fn(&mut Scope::new(), &tmp_ast, "template", args)?)
}
