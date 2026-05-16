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

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rhai::Dynamic;
    use rhai::Engine;
    use rhai::module_resolvers::FileModuleResolver;

    use super::rhai_call_template_function;

    fn fixtures_path() -> String {
        format!("{}/src/component_syntax/fixtures", env!("CARGO_MANIFEST_DIR"))
    }

    fn engine_with_fixtures() -> Engine {
        let mut engine = Engine::new();

        engine.set_module_resolver(FileModuleResolver::new_with_path(fixtures_path()));

        engine
    }

    #[test]
    fn returns_error_when_component_module_cannot_be_resolved() -> Result<()> {
        let engine = engine_with_fixtures();
        let result = rhai_call_template_function(
            &engine,
            "DoesNotExist",
            (Dynamic::UNIT, Dynamic::UNIT, Dynamic::UNIT),
        );

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn returns_error_when_module_has_no_template_function() -> Result<()> {
        let engine = engine_with_fixtures();
        let result = rhai_call_template_function(
            &engine,
            "NoTemplate",
            (Dynamic::UNIT, Dynamic::UNIT, Dynamic::UNIT),
        );

        assert!(result.is_err());

        Ok(())
    }
}
