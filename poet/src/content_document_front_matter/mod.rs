pub mod collection_placement;
pub mod collection_placement_list;

use chrono::DateTime;
use chrono::Utc;
use rhai::CustomType;
use rhai::Map;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::author_basename::AuthorBasename;
use crate::content_document_front_matter::collection_placement_list::CollectionPlacementList;

fn default_render() -> bool {
    true
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Excerpt {
//     #[serde(rename = "type")]
//     pub excerpt_type: String,
//     pub content: String,
// }

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContentDocumentFrontMatter {
    #[serde(default)]
    pub authors: Vec<AuthorBasename>,
    pub description: String,
    #[serde(default)]
    pub id: Option<String>,
    pub layout: String,
    // pub references: Vec<String>,
    // pub truth_source_for: Vec<String>,
    #[serde(default, rename = "collection")]
    pub collections: CollectionPlacementList,
    // pub excerpts: Vec<Excerpt>,
    #[serde(default, with = "crate::flexible_datetime")]
    pub last_updated_at: Option<DateTime<Utc>>,
    pub primary_collection: Option<String>,
    #[serde(default)]
    pub props: Map,
    #[serde(default = "default_render")]
    pub render: bool,
    pub title: String,
}

impl ContentDocumentFrontMatter {
    #[cfg(test)]
    pub fn mock(name: &str) -> Self {
        Self {
            authors: vec![],
            description: "".to_string(),
            id: None,
            last_updated_at: None,
            layout: "SomeLayout".to_string(),
            collections: Default::default(),
            primary_collection: None,
            props: Default::default(),
            render: true,
            title: name.to_string(),
        }
    }
}

impl ContentDocumentFrontMatter {
    fn rhai_description(&mut self) -> String {
        self.description.clone()
    }

    fn rhai_props(&mut self) -> Map {
        self.props.clone()
    }

    fn rhai_render(&mut self) -> bool {
        self.render
    }

    fn rhai_title(&mut self) -> String {
        self.title.clone()
    }
}

impl CustomType for ContentDocumentFrontMatter {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("ContentDocumentFrontMatter")
            .with_get("description", Self::rhai_description)
            .with_get("props", Self::rhai_props)
            .with_get("render", Self::rhai_render)
            .with_get("title", Self::rhai_title);
    }
}
