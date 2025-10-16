use std::sync::Arc;
use std::sync::atomic;

use actix_web::rt;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::holder::Holder as _;
use crate::mcp::resource::Resource;
use crate::mcp::resource_content::ResourceContent;
use crate::mcp::resource_content::TextResourceContent;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template_provider::ResourceTemplateProvider;

#[derive(Clone, Default)]
pub struct McpResourceProviderMarkdownPages(pub BuildProjectResultHolder);

impl ResourceTemplateProvider for McpResourceProviderMarkdownPages {
    fn mime_type(&self) -> String {
        "text/markdown".to_string()
    }

    fn resource_class(&self) -> String {
        "content".to_string()
    }

    fn resource_scheme(&self) -> String {
        "poet".to_string()
    }
}

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
                let basename = markdown_document_source.reference.basename();

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
                    uri: self.resource_uri(&basename),
                    name: basename,
                }
            })
            .collect())
    }

    async fn read_resource_contents(
        &self,
        ResourceReference {
            path, uri_string, ..
        }: ResourceReference,
    ) -> Result<Option<ResourceContentParts>> {
        let build_project_result = self.0.must_get_build_project_result().await?;

        match build_project_result.markdown_document_sources.get(&path) {
            Some(markdown_document_source) => Ok(Some(ResourceContentParts {
                parts: vec![ResourceContent::Text(TextResourceContent {
                    mime_type: self.mime_type(),
                    text: markdown_document_source.file_entry.contents.clone(),
                    uri: uri_string.clone(),
                })],
                title: markdown_document_source
                    .reference
                    .front_matter
                    .title
                    .clone(),
                uri: uri_string.clone(),
            })),
            None => Ok(None),
        }
    }

    async fn resource_update_notifier(
        self: Arc<Self>,
        cancellation_token: CancellationToken,
        resource_reference: ResourceReference,
    ) -> Result<Option<Arc<Notify>>> {
        let build_project_result_holder = self.0.clone();
        let build_update_notifier = self.0.update_notifier.clone();
        let resource_update_notifier: Arc<Notify> = Default::default();

        let resource_update_notifier_clone = resource_update_notifier.clone();
        let this = self.clone();

        rt::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => break,
                    _ = build_update_notifier.notified() => {
                        if let Some(build_project_result) = build_project_result_holder.get().await {
                            for markdown_document_source in build_project_result.changed_since_last_build {
                                let reference_uri = this.resource_uri(&markdown_document_source.relative_path);

                                if reference_uri == resource_reference.uri_string {
                                    resource_update_notifier_clone.notify_waiters();
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(Some(resource_update_notifier))
    }

    fn total(&self) -> usize {
        self.0.total.load(atomic::Ordering::Relaxed)
    }
}
