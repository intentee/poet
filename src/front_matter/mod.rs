pub mod collection_placement;
pub mod collection_placement_list;

use rhai::CustomType;
use rhai::Map;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::front_matter::collection_placement_list::CollectionPlacementList;

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Excerpt {
//     #[serde(rename = "type")]
//     pub excerpt_type: String,
//     pub content: String,
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrontMatter {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub id: Option<String>,
    pub layout: String,
    // pub references: Vec<String>,
    // pub truth_source_for: Vec<String>,
    #[serde(default, rename = "collection")]
    pub collections: CollectionPlacementList,
    // pub excerpts: Vec<Excerpt>,
    #[serde(default)]
    pub props: Map,
    pub title: String,
}

impl FrontMatter {
    #[cfg(test)]
    pub fn mock(name: &str) -> Self {
        Self {
            description: "".to_string(),
            id: None,
            layout: "SomeLayout".to_string(),
            collections: Default::default(),
            props: Default::default(),
            title: name.to_string(),
        }
    }

    fn rhai_collections(&mut self) -> CollectionPlacementList {
        self.collections.clone()
    }

    fn rhai_description(&mut self) -> String {
        self.description.clone()
    }

    fn rhai_title(&mut self) -> String {
        self.title.clone()
    }
}

impl CustomType for FrontMatter {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("FrontMatter")
            .with_get("collections", Self::rhai_collections)
            .with_get("description", Self::rhai_description)
            .with_get("title", Self::rhai_title);
    }
}
