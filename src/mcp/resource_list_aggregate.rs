use anyhow::Result;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::provides_resources::ProvidesResources;
use crate::mcp::resource::Resource;

// const PER_PAGE = 100;

pub struct ResourceListAggregate {
    pub providers: Vec<Box<dyn ProvidesResources>>,
}

impl ResourceListAggregate {
    pub async fn list_resources(
        &self,
        ListResourcesParams {
            cursor: ListResourcesCursor { offset },
        }: ListResourcesParams,
    ) -> Result<Vec<Resource>> {
        Ok(vec![])
    }
}
