pub mod collection_placement;
pub mod collection_placement_list;

use rhai::Map;
use serde::Deserialize;
use serde::Serialize;

use crate::front_matter::collection_placement_list::CollectionPlacementList;

fn default_render() -> bool {
    true
}

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
    #[serde(default = "default_render")]
    pub render: bool,
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
            render: true,
            title: name.to_string(),
        }
    }
}
