use std::collections::BTreeMap;
use std::sync::Arc;

use poet_mcp::resource_provider::ResourceProvider;

use crate::in_memory_resource_provider::InMemoryResourceProvider;

#[must_use]
pub fn resource_provider_with_paths(
    resource_class: &str,
    resource_paths: &[&str],
) -> Arc<dyn ResourceProvider> {
    Arc::new(InMemoryResourceProvider {
        resource_class: resource_class.to_owned(),
        resource_texts: resource_paths
            .iter()
            .map(|resource_path| ((*resource_path).to_owned(), (*resource_path).to_owned()))
            .collect::<BTreeMap<String, String>>(),
        resource_update_notifier: Arc::default(),
    })
}
