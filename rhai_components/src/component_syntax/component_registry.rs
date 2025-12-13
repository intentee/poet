use dashmap::DashMap;

use super::component_reference::ComponentReference;

type SmartString = smartstring::SmartString<smartstring::LazyCompact>;

pub struct ComponentRegistry {
    pub components: DashMap<SmartString, ComponentReference>,
}

impl ComponentRegistry {
    pub fn register_component(&self, component_reference: ComponentReference) {
        self.components
            .insert(component_reference.name.clone(), component_reference);
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self {
            components: DashMap::new(),
        }
    }
}
