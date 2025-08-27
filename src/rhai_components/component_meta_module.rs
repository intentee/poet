use std::sync::Arc;

use anyhow::Result;
use rhai::Engine;
use rhai::Module;
use rhai::Scope;

use super::component_reference::ComponentReference;
use super::component_registry::ComponentRegistry;
use crate::rhai_safe_random_affix::rhai_safe_random_affix;

pub struct ComponentMetaModule {
    component_registry: Arc<ComponentRegistry>,
}

impl ComponentMetaModule {
    pub fn into_global_module(self, engine: &Engine) -> Result<Module> {
        let mut meta_script = String::new();

        for entry in &self.component_registry.components {
            let affix = rhai_safe_random_affix();
            let ComponentReference {
                file_entry: _,
                global_fn_name,
                name,
                path,
            } = entry.value();

            meta_script.push_str(&format!(
                r#"
                    fn {global_fn_name}(context, props, content) {{
                        import "{path}" as {name}_{affix};

                        {name}_{affix}::template(context, props, content)
                    }}
                "#
            ));
        }

        let meta_module_ast = engine.compile(meta_script)?;

        Ok(Module::eval_ast_as_new(
            Scope::new(),
            &meta_module_ast,
            engine,
        )?)
    }
}

impl From<Arc<ComponentRegistry>> for ComponentMetaModule {
    fn from(component_registry: Arc<ComponentRegistry>) -> Self {
        Self { component_registry }
    }
}
