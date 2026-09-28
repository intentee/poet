use std::sync::Arc;

use dashmap::DashSet;
use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone, Default)]
pub struct DummyAssetCollection {
    pub assets: Arc<DashSet<String>>,
}

impl DummyAssetCollection {
    fn rhai_add(&mut self, asset: String) {
        self.assets.insert(asset);
    }
}

impl CustomType for DummyAssetCollection {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("DummyAssetCollection")
            .with_fn("add", Self::rhai_add);
    }
}
