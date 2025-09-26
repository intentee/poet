use std::cmp::Ordering;

use crate::mcp::resource_provider::ResourceProvider;

pub struct ResourceProviderHandler(pub Box<dyn ResourceProvider>);

impl PartialEq for ResourceProviderHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0.id() == other.0.id()
    }
}

impl Eq for ResourceProviderHandler {}

impl PartialOrd for ResourceProviderHandler {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResourceProviderHandler {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.id().cmp(&other.0.id())
    }
}
