use std::collections::BTreeSet;
use std::sync::Arc;

use dashmap::DashSet;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::external_asset::ExternalAsset;

#[derive(Clone, Default)]
pub struct ExternalAssetCollection {
    external_assets: Arc<DashSet<ExternalAsset>>,
}

impl ExternalAssetCollection {
    pub fn add(&self, external_asset: ExternalAsset) {
        self.external_assets.insert(external_asset);
    }

    #[must_use]
    pub fn render(&self, path_renderer: &AssetPathRenderer) -> BTreeSet<String> {
        self.external_assets
            .iter()
            .map(|external_asset| external_asset.render(path_renderer))
            .collect()
    }
}
