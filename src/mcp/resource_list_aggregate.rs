use anyhow::Result;

use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::resource::Resource;

pub struct ResourceListAggregate {}

impl ResourceListAggregate {
    pub async fn list_resources(
        &self,
        ListResourcesParams { cursor }: ListResourcesParams,
    ) -> Result<Vec<Resource>> {
        Ok(vec![])
    }
}
