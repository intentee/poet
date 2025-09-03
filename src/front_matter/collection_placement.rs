use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CollectionPlacement {
    #[serde(default)]
    pub after: Option<String>,
    pub name: String,
    #[serde(default)]
    pub parent: Option<String>,
}
