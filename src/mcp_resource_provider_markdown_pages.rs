use std::sync::atomic;

use anyhow::Result;
use async_trait::async_trait;

use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;
use crate::mcp::jsonrpc::response::success::resources_read::TextResourceContent;
use crate::mcp::resource::Resource;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

#[derive(Clone)]
pub struct McpResourceProviderMarkdownPages(pub BuildProjectResultHolder);

#[async_trait]
impl ResourceProvider for McpResourceProviderMarkdownPages {
    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>> {
        Ok(self
            .0
            .must_get_build_project_result()
            .await?
            .markdown_document_sources
            .values()
            .skip(offset)
            .take(limit)
            .map(|markdown_document_source| {
                let relative_path = &markdown_document_source.relative_path;

                Resource {
                    description: markdown_document_source
                        .reference
                        .front_matter
                        .description
                        .to_owned(),
                    title: markdown_document_source
                        .reference
                        .front_matter
                        .title
                        .to_owned(),
                    uri: self.resource_uri(relative_path),
                    name: relative_path.to_string(),
                }
            })
            .collect())
    }

    async fn read_resource_contents(
        &self,
        resource_uri: String,
        resource_path: String,
    ) -> Result<Option<Vec<ResourceContent>>> {
        let build_project_result = self.0.must_get_build_project_result().await?;

        match build_project_result
            .markdown_document_sources
            .get(&resource_path)
        {
            Some(markdown_document_source) => {
                Ok(Some(vec![ResourceContent::Text(TextResourceContent {
                    meta: None,
                    mime_type: "text/markdown".to_string(),
                    text: markdown_document_source.file_entry.contents.clone(),
                    uri: resource_uri,
                })]))
            }
            None => Ok(None),
        }
    }

    fn resource_class(&self) -> String {
        "content".to_string()
    }

    fn total(&self) -> usize {
        self.0.total.load(atomic::Ordering::Relaxed)
    }
}
