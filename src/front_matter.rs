use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Excerpt {
    #[serde(rename = "type")]
    pub excerpt_type: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FrontMatter {
    pub created_at: NaiveDate,
    pub layout: String,
    pub references: Vec<String>,
    pub truth_source_for: Vec<String>,
    pub collections: Vec<Collection>,
    pub excerpts: Vec<Excerpt>,
}
