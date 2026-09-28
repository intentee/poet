use serde::Deserialize;
use serde::Serialize;

use crate::resource_content::ResourceContent;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmbeddedResource {
    pub resource: ResourceContent,
}
