use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use nanoid::nanoid;

use super::component_reference::ComponentReference;

// // Allow only characters that are safe for use in function names
// const SUFFIX_ALPHABET: [char; 62] = [
//     '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
//     'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
//     'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
//     'V', 'W', 'X', 'Y', 'Z',
// ];

pub struct ComponentRegsitry {
    pub components: DashMap<String, ComponentReference>,
}

impl ComponentRegsitry {
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
}

impl Default for ComponentRegsitry {
    fn default() -> Self {
        Self {
            components: DashMap::new(),
        }
    }
}
