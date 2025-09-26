use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resource {
    pub description: String,
    pub name: String,
    pub title: String,
}
