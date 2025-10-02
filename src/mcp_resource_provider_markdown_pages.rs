use std::sync::atomic;

use actix_web::rt;
use anyhow::Result;
use async_trait::async_trait;
use http::Uri;
use log::error;
use log::warn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::holder::Holder as _;
use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;
use crate::mcp::jsonrpc::response::success::resources_read::TextResourceContent;
use crate::mcp::resource::Resource;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template_provider::ResourceTemplateProvider;

fn read_resource_contents(
    build_project_result: BuildProjectResult,
    mime_type: String,
    path: String,
    uri: Uri,
) -> Result<Option<ResourceContentParts>> {
    match build_project_result.markdown_document_sources.get(&path) {
        Some(markdown_document_source) => Ok(Some(ResourceContentParts {
            parts: vec![ResourceContent::Text(TextResourceContent {
                meta: None,
                mime_type,
                text: markdown_document_source.file_entry.contents.clone(),
                uri: uri.to_string(),
            })],
            title: markdown_document_source
                .reference
                .front_matter
                .title
                .clone(),
            uri: uri.to_string(),
        })),
        None => Ok(None),
    }
}

#[derive(Clone)]
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
        ResourceReference {
            class: _,
            path,
            scheme: _,
            uri,
        }: ResourceReference,
    ) -> Result<Option<ResourceContentParts>> {
        let build_project_result = self.0.must_get_build_project_result().await?;

        read_resource_contents(build_project_result, self.mime_type(), path, uri)
    }

    async fn subscribe(
        &self,
        ResourceReference {
            class: _,
            path,
            scheme: _,
            uri,
        }: ResourceReference,
    ) -> Result<Option<Receiver<ResourceContentParts>>> {
        let (resource_content_parts_tx, resource_content_parts_rx) = mpsc::channel(3);
        let build_project_result_holder = self.0.clone();
        let mime_type = self.mime_type();

        rt::spawn(async move {
            loop {
                build_project_result_holder.update_notifier.notified().await;

                match build_project_result_holder.get().await {
                    Some(build_project_result) => {
                        if let Err(err) = resource_content_parts_tx.send(match read_resource_contents(
                            build_project_result,
                            mime_type.clone(),
                            path.clone(),
                            uri.clone(),
                        ) {
                            Ok(Some(resource_content_parts)) => resource_content_parts,
                            Ok(None) => {
                                warn!("Resource has been removed while being subscribed to: '{path}'");
                                break;
                            }
                            Err(err) => {
                                error!("Unable to get resource content parts for '{path}': {err:#?}");
                                break;
                            }
                        }).await {
                            error!("Unable to forward resource update: {err:#?}");
                        }
                    },
                    None => break,
                }
            }
        });

        Ok(Some(resource_content_parts_rx))
    }

    fn total(&self) -> usize {
        self.0.total.load(atomic::Ordering::Relaxed)
    }
}
