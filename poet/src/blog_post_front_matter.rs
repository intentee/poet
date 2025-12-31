use serde::Deserialize;
use serde::Serialize;

fn default_render() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlogPostFrontMatter {
    #[serde(default)]
    pub authors: Vec<String>,
    pub description: String,
    pub layout: String,
    #[serde(default = "default_render")]
    pub render: bool,
    pub title: String,
}
