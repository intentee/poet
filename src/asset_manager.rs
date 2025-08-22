use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct AssetManager {}

impl CustomType for AssetManager {
    fn build(mut builder: TypeBuilder<Self>) {}
}
