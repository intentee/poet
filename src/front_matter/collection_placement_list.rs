use rhai::CustomType;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::front_matter::collection_placement::CollectionPlacement;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CollectionPlacementList {
    pub placements: Vec<CollectionPlacement>,
}

impl CollectionPlacementList {
    fn rhai_has(&mut self, name: String) -> bool {
        self.placements
            .iter()
            .any(|collection_placement| collection_placement.name == name)
    }
}

impl CustomType for CollectionPlacementList {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("CollectionPlacementList")
            .with_fn("has", Self::rhai_has);
    }
}
