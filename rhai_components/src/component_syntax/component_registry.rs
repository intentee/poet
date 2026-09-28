use dashmap::DashSet;

use super::component_reference::ComponentReference;

#[derive(Default)]
pub struct ComponentRegistry {
    pub components: DashSet<ComponentReference>,
}

impl ComponentRegistry {
    pub fn register_component(&self, component_reference: ComponentReference) {
        self.components.insert(component_reference);
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentReference;
    use super::ComponentRegistry;

    #[test]
    fn finds_registered_component_by_name() {
        let registry = ComponentRegistry::default();

        registry.register_component(ComponentReference {
            name: "Note".to_owned(),
        });

        assert!(registry.components.contains("Note"));
    }
}
