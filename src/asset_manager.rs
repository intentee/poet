use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct AssetManager {}

impl CustomType for AssetManager {
    fn build(_builder: TypeBuilder<Self>) {}
}

impl Default for AssetManager {
    fn default() -> Self {
        AssetManager {}
    }
}
