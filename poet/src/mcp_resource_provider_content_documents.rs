use std::sync::Arc;

use actix_web::rt;
use async_trait::async_trait;
use poet_content::build_project_result::BuildProjectResult;
use poet_content::content_document_basename::ContentDocumentBasename;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource::Resource;
use poet_mcp::resource_content::ResourceContent;
use poet_mcp::resource_provider::ResourceProvider;
use poet_mcp::resource_provider_list_params::ResourceProviderListParams;
use poet_mcp::resource_reference::ResourceReference;
use poet_mcp::resource_template_provider::ResourceTemplateProvider;
use poet_mcp::text_resource_content::TextResourceContent;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;

#[derive(Clone, Default)]
pub struct McpResourceProviderContentDocuments {
    pub build_project_result_holder: Holder<BuildProjectResult>,
}

impl McpResourceProviderContentDocuments {
    fn build_project_result(&self) -> Result<BuildProjectResult, PoetError> {
        self.build_project_result_holder
            .get()
            .ready_or(PoetError::BuildProjectResultNotReady)
    }

    fn is_updated_by(
        resource_reference: &ResourceReference,
        build_project_result: &BuildProjectResult,
    ) -> bool {
        let subscribed_basename = ContentDocumentBasename(resource_reference.path.clone());

        build_project_result
            .changed_since_last_build
            .iter()
            .any(|content_document_source| {
                content_document_source.reference.basename() == subscribed_basename
            })
    }
}

impl ResourceTemplateProvider for McpResourceProviderContentDocuments {
    fn mime_type(&self) -> String {
        "text/markdown".to_owned()
    }

    fn resource_class(&self) -> String {
        "content".to_owned()
    }

    fn resource_scheme(&self) -> String {
        "poet".to_owned()
    }
}

#[async_trait]
impl ResourceProvider for McpResourceProviderContentDocuments {
    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>, ProviderError> {
        Ok(self
            .build_project_result()?
            .content_document_sources
            .values()
            .skip(offset)
            .take(limit)
            .map(|content_document_source| {
                let basename_string: String =
                    content_document_source.reference.basename().to_string();

                Resource {
                    description: content_document_source
                        .reference
                        .front_matter
                        .description
                        .clone(),
                    title: content_document_source.reference.front_matter.title.clone(),
                    uri: self.resource_uri(&basename_string),
                    name: basename_string,
                }
            })
            .collect())
    }

    async fn read_resource_contents(
        &self,
        ResourceReference {
            path, uri_string, ..
        }: ResourceReference,
    ) -> Result<Option<Vec<ResourceContent>>, ProviderError> {
        Ok(self
            .build_project_result()?
            .content_document_sources
            .get(&ContentDocumentBasename(path))
            .map(|content_document_source| {
                vec![ResourceContent::Text(TextResourceContent {
                    mime_type: self.mime_type(),
                    text: content_document_source.file_entry.contents.clone(),
                    uri: uri_string,
                })]
            }))
    }

    fn resource_update_notifier(
        self: Arc<Self>,
        cancellation_token: CancellationToken,
        resource_reference: ResourceReference,
    ) -> Arc<Notify> {
        let build_project_result_holder = self.build_project_result_holder.clone();
        let mut build_project_result_updates = self.build_project_result_holder.subscribe();
        let resource_update_notifier: Arc<Notify> = Arc::default();
        let notified_resource_update_notifier = resource_update_notifier.clone();

        rt::spawn(async move {
            loop {
                tokio::select! {
                    () = cancellation_token.cancelled() => break,
                    Ok(()) = build_project_result_updates.changed() => {
                        if let HolderState::Ready(build_project_result) = build_project_result_holder.get()
                            && Self::is_updated_by(&resource_reference, &build_project_result)
                        {
                            notified_resource_update_notifier.notify_one();
                        }
                    }
                }
            }
        });

        resource_update_notifier
    }

    fn total(&self) -> usize {
        match self.build_project_result_holder.get() {
            HolderState::Ready(BuildProjectResult {
                content_document_sources,
                ..
            }) => content_document_sources.len(),
            HolderState::NotReady => 0,
        }
    }
}
