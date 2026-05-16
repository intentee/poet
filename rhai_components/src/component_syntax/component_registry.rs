use dashmap::DashMap;

use super::component_reference::ComponentReference;

pub struct ComponentRegistry {
    pub components: DashMap<String, ComponentReference>,
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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::ComponentReference;
    use super::ComponentRegistry;

    #[test]
    fn default_starts_empty_and_register_inserts_by_name() -> Result<()> {
        let registry = ComponentRegistry::default();

        assert_eq!(registry.components.len(), 0);

        registry.register_component(ComponentReference {
            name: "Note".to_string(),
            path: "shortcodes/Note".to_string(),
        });

        assert_eq!(registry.components.len(), 1);

        assert!(registry.components.get("Note").is_some_and(|entry| {
            entry.value().name == "Note" && entry.value().path == "shortcodes/Note"
        }));

        Ok(())
    }
}
