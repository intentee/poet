use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use log::error;
use log::info;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::build_project;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::watch::service::Service;
use crate::filesystem::storage::Storage;
use crate::holder::Holder as _;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::notification::resources_list_changed::ResourcesListChanged;
use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::session_manager::SessionManager;
use crate::mcp_resource_provider_markdown_pages::McpResourceProviderMarkdownPages;
use crate::rhai_template_renderer_holder::RhaiTemplateRendererHolder;

pub struct ProjectBuilder {
    pub addr: SocketAddr,
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
    pub mcp_resource_provider_markdown_pages: Arc<McpResourceProviderMarkdownPages>,
    pub on_content_file_changed: Arc<Notify>,
    pub rhai_template_renderer_holder: RhaiTemplateRendererHolder,
    pub session_manager: SessionManager,
    pub source_filesystem: Arc<Storage>,
}

impl ProjectBuilder {
    async fn do_build_project(&self) {
        let rhai_template_renderer = match self.rhai_template_renderer_holder.get().await {
            Some(rhai_template_renderer) => rhai_template_renderer,
            None => {
                debug!("Rhai components are not compiled yet. Skipping build");

                return;
            }
        };
        let base_path = format!("http://{}/", self.addr);

        match build_project(
            AssetPathRenderer {
                base_path: base_path.clone(),
            },
            base_path,
            true,
            rhai_template_renderer,
            self.source_filesystem.clone(),
        )
        .await
        {
            Ok(build_project_result_stub) => {
                self.build_project_result_holder
                    .set(Some(
                        if let Some(old_build_project_result) =
                            self.build_project_result_holder.get().await
                        {
                            build_project_result_stub.changed_compared_to(old_build_project_result)
                        } else {
                            build_project_result_stub.into()
                        },
                    ))
                    .await;

                if let Err(err) = self
                    .session_manager
                    .broadcast(ServerToClientNotification::ResourcesListChanged(
                        ResourcesListChanged {
                            jsonrpc: JSONRPC_VERSION.to_string(),
                        },
                    ))
                    .await
                {
                    error!("Failed to notify MCP sessions: {err:#?}");
                }

                info!("Build successful");
            }
            Err(err) => error!("Failed to build project: {err:#}"),
        }
    }
}

#[async_trait]
impl Service for ProjectBuilder {
    async fn run(&self) -> Result<()> {
        loop {
            self.do_build_project().await;

            tokio::select! {
                _ = self.on_content_file_changed.notified() => continue,
                _ = self.rhai_template_renderer_holder.update_notifier.notified() => continue,
                _ = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
