use serde::Deserialize;
use serde::Serialize;

use crate::resource_template::ResourceTemplate;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesTemplatesListResult {
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<ResourceTemplate>,
}
