// use chrono::NaiveDate;
use rhai::CustomType;
use rhai::Map;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Collection {
//     pub name: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub after: Option<String>,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Excerpt {
//     #[serde(rename = "type")]
//     pub excerpt_type: String,
//     pub content: String,
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FrontMatter {
    // pub created_at: NaiveDate,
    #[serde(default)]
    pub description: String,
    pub layout: String,
    // pub references: Vec<String>,
    // pub truth_source_for: Vec<String>,
    // pub collections: Vec<Collection>,
    // pub excerpts: Vec<Excerpt>,
    #[serde(default)]
    pub props: Map,
    pub title: String,
}

impl FrontMatter {
    pub fn get_description(&mut self) -> String {
        self.description.clone()
    }

    pub fn get_title(&mut self) -> String {
        self.title.clone()
    }
}

impl CustomType for FrontMatter {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("FrontMatter")
            .with_get("description", Self::get_description)
            .with_get("title", Self::get_title);
    }
}
