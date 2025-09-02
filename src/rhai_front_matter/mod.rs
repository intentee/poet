pub mod rhai_collection_placement_list;

use std::collections::HashSet;
use std::sync::Arc;

use rhai::CustomType;
use rhai::TypeBuilder;

use crate::front_matter::FrontMatter;
use crate::rhai_front_matter::rhai_collection_placement_list::RhaiCollectionPlacementList;

#[derive(Clone, Debug)]
pub struct RhaiFrontMatter {
    pub available_collections: Arc<HashSet<String>>,
    pub front_matter: FrontMatter,
}

impl RhaiFrontMatter {
    fn rhai_collections(&mut self) -> RhaiCollectionPlacementList {
        RhaiCollectionPlacementList {
            available_collections: self.available_collections.clone(),
            collection_placement_list: self.front_matter.collections.clone(),
        }
    }

    fn rhai_description(&mut self) -> String {
        self.front_matter.description.clone()
    }

    fn rhai_title(&mut self) -> String {
        self.front_matter.title.clone()
    }
}

impl CustomType for RhaiFrontMatter {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiFrontMatter")
            .with_get("collections", Self::rhai_collections)
            .with_get("description", Self::rhai_description)
            .with_get("title", Self::rhai_title);
    }
}
