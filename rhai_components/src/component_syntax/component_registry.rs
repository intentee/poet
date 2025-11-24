use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;

use super::component_reference::ComponentReference;
use crate::component_syntax::component_reference_stub::ComponentReferenceStub;
use crate::rhai_safe_random_affix::rhai_safe_random_affix;

pub struct ComponentRegistry {
    pub components: DashMap<String, ComponentReference>,
}

impl ComponentRegistry {
    pub fn get_global_fn_name(&self, component_name: &str) -> Result<String> {
        self.components
            .get(component_name)
            .map(|comp_ref| comp_ref.global_fn_name.clone())
            .ok_or_else(|| anyhow!("Component '{component_name}' not found"))
    }

    pub fn register_component(&self, component_reference: ComponentReference) {
        self.components
            .insert(component_reference.name.clone(), component_reference);
    }

    pub fn register_component_from_stub(
        &self,
        ComponentReferenceStub { name, path }: ComponentReferenceStub,
    ) {
        self.register_component(ComponentReference {
            global_fn_name: format!("{}_{}", name, rhai_safe_random_affix()),
            name,
            path,
        });
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self {
            components: DashMap::new(),
        }
    }
}
