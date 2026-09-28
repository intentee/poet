use crate::maximum_component_nesting_depth::MAXIMUM_COMPONENT_NESTING_DEPTH;
use crate::rhai_components_error::RhaiComponentsError;

#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentNestingDepth {
    pub depth: usize,
}

impl ComponentNestingDepth {
    pub fn nested(self, component_name: String) -> Result<Self, RhaiComponentsError> {
        if self.depth < MAXIMUM_COMPONENT_NESTING_DEPTH {
            Ok(Self {
                depth: self.depth + 1,
            })
        } else {
            Err(RhaiComponentsError::ComponentNestingTooDeep {
                component_name,
                maximum_depth: MAXIMUM_COMPONENT_NESTING_DEPTH,
            })
        }
    }
}
