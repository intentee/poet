use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use rhai::Engine;
use rhai::Position;

use crate::component_syntax::component_meta_module::ComponentMetaModule;
use crate::component_syntax::component_reference::ComponentReference;
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

        let templates: DashMap<String, ComponentReference> = DashMap::new();

        for entry in &self.component_registry().components {
            let component_reference = entry.value();

            let module_resolver = engine.module_resolver();
            let module = module_resolver.resolve(
                &engine,
                None,
                &component_reference.path,
                Position::NONE,
            )?;

            engine.register_static_module(component_reference.name.clone(), module);

            templates.insert(
                component_reference.name.clone(),
                component_reference.clone(),
            );
        }

        let meta_module = ComponentMetaModule::from(self.component_registry());

        engine.register_global_module(meta_module.into_global_module(&engine)?.into());

        self.prepare_engine(&mut engine)?;

        Ok(engine)
    }
}
