use std::collections::HashSet;
use std::sync::Arc;

use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::front_matter::collection_placement_list::CollectionPlacementList;

#[derive(Clone, Debug)]
pub struct RhaiCollectionPlacementList {
    pub available_collections: Arc<HashSet<String>>,
    pub collection_placement_list: CollectionPlacementList,
}

impl RhaiCollectionPlacementList {
    fn rhai_has(&mut self, name: String) -> Result<bool, Box<EvalAltResult>> {
        if !self.available_collections.contains(&name) {
            return Err(format!("Collection is never used in any document: '{name}'").into());
        }

        Ok(self
            .collection_placement_list
            .placements
            .iter()
            .any(|collection_placement| collection_placement.name == name))
    }
}

impl CustomType for RhaiCollectionPlacementList {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiCollectionPlacementList")
            .with_fn("has", Self::rhai_has);
    }
}
