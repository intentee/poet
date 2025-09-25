use std::collections::BTreeSet;

use anyhow::Result;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::provides_resources::ProvidesResources;
use crate::mcp::resource::Resource;

const PER_PAGE: usize = 100;

#[derive(Default)]
pub struct ResourceListAggregate {
    /// Providers need to be sorted for the offset to work
    providers: BTreeSet<Box<dyn ProvidesResources>>,
}

impl ResourceListAggregate {
    pub async fn list_resources(
        &self,
        ListResourcesParams {
            cursor: ListResourcesCursor { offset },
        }: ListResourcesParams,
    ) -> Result<Vec<Resource>> {
        let mut skipped_total: usize = 0;
        let mut resources: Vec<Resource> = vec![];

        for provider in &self.providers {
            let provider_total = provider.total();

            if skipped_total + provider_total < offset {
                skipped_total += provider_total;
                continue;
            } else if resources.len() < PER_PAGE {
                // Something left in this provider
                let provider_offset = offset - skipped_total;
                let mut taken_resources = provider
                    .list_resources(provider_offset, PER_PAGE - resources.len())
                    .await?;

                skipped_total += taken_resources.len();

                resources.append(&mut taken_resources);
            } else {
                break;
            }
        }

        Ok(resources)
    }
}
