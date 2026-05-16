use std::sync::Arc;

use anyhow::Result;
use rhai::Engine;

use crate::component_syntax::component_registry::ComponentRegistry;
use crate::component_syntax::evaluator_factory::EvaluatorFactory;
use crate::component_syntax::parse_component::parse_component;
use crate::rhai_helpers::clsx;
use crate::rhai_helpers::error;
use crate::rhai_helpers::has;

pub trait BuildsEngine {
    fn component_registry(&self) -> Arc<ComponentRegistry>;

    fn prepare_engine(&self, engine: &mut Engine) -> Result<()>;

    fn create_engine(&self) -> Result<Engine> {
        let evaluator_factory = EvaluatorFactory {
            component_registry: self.component_registry().clone(),
        };

        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_call_levels(128);
        engine.set_max_expr_depths(256, 256);

        engine.register_fn("clsx", clsx);
        engine.register_fn("error", error);
        engine.register_fn("has", has);

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            evaluator_factory.create_component_evaluator(),
        );

        self.prepare_engine(&mut engine)?;

        Ok(engine)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use anyhow::anyhow;
    use rhai::Engine;
    use rhai::module_resolvers::FileModuleResolver;

    use super::BuildsEngine;
    use super::ComponentRegistry;
    use crate::component_syntax::component_reference::ComponentReference;

    fn fixtures_path() -> String {
        format!("{}/src/component_syntax/fixtures", env!("CARGO_MANIFEST_DIR"))
    }

    struct TestEngineOk {
        registry: Arc<ComponentRegistry>,
    }

    impl BuildsEngine for TestEngineOk {
        fn component_registry(&self) -> Arc<ComponentRegistry> {
            self.registry.clone()
        }

        fn prepare_engine(&self, engine: &mut Engine) -> Result<()> {
            engine.set_module_resolver(FileModuleResolver::new_with_path(fixtures_path()));

            Ok(())
        }
    }

    struct TestEngineFailingPrepare {
        registry: Arc<ComponentRegistry>,
    }

    impl BuildsEngine for TestEngineFailingPrepare {
        fn component_registry(&self) -> Arc<ComponentRegistry> {
            self.registry.clone()
        }

        fn prepare_engine(&self, _engine: &mut Engine) -> Result<()> {
            Err(anyhow!("prepare_engine failed on purpose"))
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

    #[test]
    fn create_engine_registers_helpers_and_custom_syntax() -> Result<()> {
        let builder = TestEngineOk {
            registry: registry_with(&["Note"]),
        };

        assert!(builder.create_engine().is_ok_and(|engine| engine
            .eval::<String>(r#"clsx(#{ ok: true })"#)
            .is_ok_and(|result| result == "ok")));

        Ok(())
    }

    #[test]
    fn create_engine_propagates_prepare_engine_error() -> Result<()> {
        let builder = TestEngineFailingPrepare {
            registry: registry_with(&[]),
        };

        assert!(builder.create_engine().is_err_and(|error| {
            error.to_string().contains("prepare_engine failed on purpose")
        }));

        Ok(())
    }
}
