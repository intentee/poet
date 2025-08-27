use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct RhaiComponentContext {}

impl CustomType for RhaiComponentContext {
    fn build(_builder: TypeBuilder<Self>) {}
}

impl Default for RhaiComponentContext {
    fn default() -> Self {
        RhaiComponentContext {}
    }
}
