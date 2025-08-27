use std::collections::HashMap;

use crate::rhai_context::ShortcodeRenderer;

pub struct ShortcodeCollection {
    pub shortcodes: HashMap<String, Box<ShortcodeRenderer>>,
}

impl Default for ShortcodeCollection {
    fn default() -> Self {
        Self {
            shortcodes: HashMap::new(),
        }
    }
}
