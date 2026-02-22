mod app_data;
mod http_route;
mod service;
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
use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::builds_project::BuildsProject;
use crate::cmd::handler::Handler;
use crate::cmd::service_manager::ServiceManager;
use crate::cmd::value_parser::parse_socket_addr;
use crate::cmd::value_parser::validate_is_directory;
use crate::cmd::watch::service::esbuild_metafile_reader::EsbuildMetaFileReader;
use crate::cmd::watch::service::filesystem_http_route_index_builder::FilesystemHttpRouteIndexBuilder;
use crate::cmd::watch::service::http_server::HttpServer;
use crate::cmd::watch::service::project_builder::ProjectBuilder;
use crate::cmd::watch::service::prompt_controller_collection_builder::PromptControllerCollectionBuilder;
use crate::cmd::watch::service::search_index_builder::SearchIndexBuilder;
use crate::cmd::watch::service::shortcodes_compiler::ShortcodesCompiler;
use crate::esbuild_metafile_holder::EsbuildMetaFileHolder;
use crate::filesystem_http_route_index_holder::FilesystemHttpRouteIndexHolder;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::session_manager::SessionManager;
use crate::mcp::tool_registry::ToolRegistry;
use crate::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use crate::prompt_controller_collection_holder::PromptControllerCollectionHolder;
use crate::rhai_template_renderer_holder::RhaiTemplateRendererHolder;
use crate::search_index_reader_holder::SearchIndexReaderHolder;
use crate::search_tool::SearchTool;

#[derive(Parser)]
pub struct Watch {
    #[arg(long, default_value="127.0.0.1:8050", value_parser = parse_socket_addr)]
    addr: SocketAddr,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,

    #[arg(long, default_value = "false")]
    sitemap: bool,
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
            on_author_file_changed,
            on_content_file_changed,
            on_esbuild_metafile_changed,
            on_prompt_file_changed,
            on_shortcode_file_changed,
        } = watch_project_files(self.source_directory.clone())?;

        let generated_page_base_path = format!("http://{}/", self.addr);

        let asset_path_renderer = AssetPathRenderer {
            base_path: generated_page_base_path.clone(),
        };
        let build_project_result_holder: BuildProjectResultHolder = Default::default();
        let esbuild_metafile_holder: EsbuildMetaFileHolder = Default::default();
        let filesystem_http_route_index_holder: FilesystemHttpRouteIndexHolder = Default::default();
        let mcp_resource_provider_content_documents: McpResourceProviderContentDocuments =
            McpResourceProviderContentDocuments(build_project_result_holder.clone());
        let prompt_controller_collection_holder: PromptControllerCollectionHolder =
            Default::default();
        let rhai_template_renderer_holder: RhaiTemplateRendererHolder = Default::default();
        let source_filesystem = self.source_filesystem();
        let resource_list_providers: Vec<Arc<dyn ResourceProvider>> =
            vec![Arc::new(mcp_resource_provider_content_documents.clone())];
        let search_index_reader_holder: SearchIndexReaderHolder = Default::default();
        let session_manager: SessionManager = Default::default();
        let mut tool_registry: ToolRegistry = Default::default();

        tool_registry.register_owned(SearchTool {
            mcp_resource_provider_content_documents: mcp_resource_provider_content_documents
                .clone(),
            search_index_reader_holder: search_index_reader_holder.clone(),
        });

        let mut service_manager: ServiceManager = Default::default();

        service_manager.register_service(Arc::new(EsbuildMetaFileReader {
            ctrlc_notifier: ctrlc_notifier.clone(),
            esbuild_metafile_holder: esbuild_metafile_holder.clone(),
            on_esbuild_metafile_changed,
            source_filesystem: source_filesystem.clone(),
        }));

        service_manager.register_service(Arc::new(FilesystemHttpRouteIndexBuilder {
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            filesystem_http_route_index_holder: filesystem_http_route_index_holder.clone(),
        }));

        service_manager.register_service(Arc::new(HttpServer {
            addr: self.addr,
            assets_directory: self.assets_directory(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            filesystem_http_route_index_holder,
            prompt_controller_collection_holder: prompt_controller_collection_holder.clone(),
            resource_list_aggregate: Arc::new(resource_list_providers.into()),
            session_manager: session_manager.clone(),
            tool_registry: Arc::new(tool_registry),
        }));

        service_manager.register_service(Arc::new(ProjectBuilder {
            asset_path_renderer: asset_path_renderer.clone(),
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            esbuild_metafile_holder: esbuild_metafile_holder.clone(),
            generated_page_base_path: generated_page_base_path.clone(),
            on_author_file_changed,
            on_content_file_changed,
            rhai_template_renderer_holder: rhai_template_renderer_holder.clone(),
            session_manager,
            generate_sitemap: self.sitemap,
            source_filesystem: source_filesystem.clone(),
        }));

        service_manager.register_service(Arc::new(PromptControllerCollectionBuilder {
            asset_path_renderer,
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            esbuild_metafile_holder,
            on_prompt_file_changed,
            prompt_controller_collection_holder,
            rhai_template_renderer_holder: rhai_template_renderer_holder.clone(),
            source_filesystem: source_filesystem.clone(),
        }));

        service_manager.register_service(Arc::new(SearchIndexBuilder {
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            search_index_reader_holder,
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
