use std::cmp::Ordering;
use std::sync::Arc;

use crate::mcp::resource_provider::ResourceProvider;

pub struct ResourceProviderOrdered(pub Arc<dyn ResourceProvider>);

impl PartialEq for ResourceProviderOrdered {
    fn eq(&self, other: &Self) -> bool {
        self.0.resource_class() == other.0.resource_class()
    }
}

impl Eq for ResourceProviderOrdered {}

impl PartialOrd for ResourceProviderOrdered {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResourceProviderOrdered {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.resource_class().cmp(&other.0.resource_class())
    }
}
