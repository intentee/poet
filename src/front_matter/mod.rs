// use chrono::NaiveDate;
use rhai::CustomType;
use rhai::Map;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    #[serde(default)]
    pub after: Option<String>,
    pub name: String,
    #[serde(default)]
    pub parent: Option<String>,
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
    // pub created_at: NaiveDate,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub id: Option<String>,
    pub layout: String,
    // pub references: Vec<String>,
    // pub truth_source_for: Vec<String>,
    #[serde(default)]
    pub collection: Vec<Collection>,
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
