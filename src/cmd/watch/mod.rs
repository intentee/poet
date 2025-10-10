mod app_data;
mod http_route;
mod resolve_generated_page;
mod respond_with_generated_page;
mod service;
mod service_manager;
mod watch_project_files;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use log::info;
use tokio_util::sync::CancellationToken;

use self::watch_project_files::WatchProjectHandle;
use self::watch_project_files::watch_project_files;
use super::Handler;
use super::value_parser::parse_socket_addr;
use super::value_parser::validate_is_directory;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::builds_project::BuildsProject;
use crate::cmd::watch::service::http_server::HttpServer;
use crate::cmd::watch::service::project_builder::ProjectBuilder;
use crate::cmd::watch::service::search_index_builder::SearchIndexBuilder;
use crate::cmd::watch::service::shortcodes_compiler::ShortcodesCompiler;
use crate::cmd::watch::service_manager::ServiceManager;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::session_manager::SessionManager;
use crate::mcp_resource_provider_markdown_pages::McpResourceProviderMarkdownPages;
use crate::rhai_template_renderer_holder::RhaiTemplateRendererHolder;

#[derive(Parser)]
pub struct Watch {
    #[arg(long, default_value="127.0.0.1:8050", value_parser = parse_socket_addr)]
    addr: SocketAddr,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,
}

impl BuildsProject for Watch {
    fn source_directory(&self) -> PathBuf {
        self.source_directory.clone()
    }
}

#[async_trait(?Send)]
impl Handler for Watch {
    async fn handle(&self) -> Result<()> {
        let ctrlc_notifier = CancellationToken::new();
        let ctrlc_notifier_handler = ctrlc_notifier.clone();

        ctrlc::set_handler(move || {
            ctrlc_notifier_handler.cancel();
        })?;

        let WatchProjectHandle {
            debouncer: _debouncer,
            on_content_file_changed,
            on_shortcode_file_changed,
        } = watch_project_files(self.source_directory.clone())?;

        let build_project_result_holder: BuildProjectResultHolder = Default::default();
        let mcp_resource_provider_markdown_pages = Arc::new(McpResourceProviderMarkdownPages(
            build_project_result_holder.clone(),
        ));
        let rhai_template_renderer_holder: RhaiTemplateRendererHolder = Default::default();
        let source_filesystem = self.source_filesystem();
        let resource_list_providers: Vec<Arc<dyn ResourceProvider>> =
            vec![mcp_resource_provider_markdown_pages.clone()];
        let session_manager = SessionManager {
            session_storage: Arc::new(Default::default()),
        };

        let mut service_manager: ServiceManager = Default::default();

        service_manager.register_service(Arc::new(HttpServer {
            addr: self.addr,
            assets_directory: self.assets_directory(),
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            resource_list_aggregate: Arc::new(resource_list_providers.try_into()?),
            session_manager: session_manager.clone(),
        }));

        service_manager.register_service(Arc::new(ProjectBuilder {
            addr: self.addr,
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            mcp_resource_provider_markdown_pages,
            on_content_file_changed,
            rhai_template_renderer_holder: rhai_template_renderer_holder.clone(),
            session_manager,
            source_filesystem: source_filesystem.clone(),
        }));

        service_manager.register_service(Arc::new(SearchIndexBuilder {
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
        }));

        service_manager.register_service(Arc::new(ShortcodesCompiler {
            ctrlc_notifier: ctrlc_notifier.clone(),
            on_shortcode_file_changed,
            rhai_template_renderer_holder: rhai_template_renderer_holder.clone(),
            source_filesystem: source_filesystem.clone(),
        }));

        service_manager.run().await?;

        info!("Poet is shutting down...");

        Ok(())
    }
}
