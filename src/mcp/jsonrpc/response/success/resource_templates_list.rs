use serde::Deserialize;
use serde::Serialize;

use crate::mcp::resource_template::ResourceTemplate;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesTemplatesList {
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<ResourceTemplate>,
}
