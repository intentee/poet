use chrono::DateTime;
use chrono::Utc;
use rhai::CustomType;
use rhai::Map;
use rhai::TypeBuilder;
use serde::Deserialize;

use crate::collection_placement::CollectionPlacement;

const fn render_by_default() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentDocumentFrontMatter {
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default, rename = "collection")]
    pub collections: Vec<CollectionPlacement>,
    pub description: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::deserialize_flexible_datetime::deserialize_flexible_datetime"
    )]
    pub last_updated_at: Option<DateTime<Utc>>,
    pub layout: String,
    #[serde(default)]
    pub primary_collection: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::deserialize_rhai_map::deserialize_rhai_map"
    )]
    pub props: Map,
    #[serde(default = "render_by_default")]
    pub render: bool,
    pub title: String,
}

impl ContentDocumentFrontMatter {
    fn rhai_description(&mut self) -> String {
        self.description.clone()
    }

    fn rhai_props(&mut self) -> Map {
        self.props.clone()
    }

    const fn rhai_render(&mut self) -> bool {
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
