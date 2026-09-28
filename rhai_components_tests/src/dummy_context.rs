use rhai::CustomType;
use rhai::TypeBuilder;

use crate::dummy_asset_collection::DummyAssetCollection;

#[derive(Clone, Default)]
pub struct DummyContext {
    pub assets: DummyAssetCollection,
}

impl DummyContext {
    fn rhai_assets(&mut self) -> DummyAssetCollection {
        self.assets.clone()
    }
}

impl CustomType for DummyContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("DummyContext")
            .with_get("assets", Self::rhai_assets);
    }
}
