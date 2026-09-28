mod app_data;
mod http_route;
mod service;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use clap::Parser;
use log::info;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::build_project_result::BuildProjectResult;
use poet_mcp::implementation::Implementation;
use poet_watcher::project_file_notifications::ProjectFileNotifications;
use poet_watcher::project_file_watcher::ProjectFileWatcher;
use poet_watcher::watch_project_files::watch_project_files;
use tokio_util::sync::CancellationToken;

use crate::cmd::builds_project::BuildsProject;
use crate::cmd::handler::Handler;
use crate::cmd::service_manager::ServiceManager;
use crate::cmd::value_parser::parse_socket_addr::parse_socket_addr;
use crate::cmd::value_parser::validate_is_directory::validate_is_directory;
use crate::cmd::watch::service::esbuild_metafile_reader::EsbuildMetafileReader;
use crate::cmd::watch::service::filesystem_http_route_index_builder::FilesystemHttpRouteIndexBuilder;
use crate::cmd::watch::service::http_server::HttpServer;
use crate::cmd::watch::service::project_builder::ProjectBuilder;
use crate::cmd::watch::service::prompt_document_controller_collection_builder::PromptDocumentControllerCollectionBuilder;
use crate::cmd::watch::service::resources_list_changed_broadcaster::ResourcesListChangedBroadcaster;
use crate::cmd::watch::service::search_index_builder::SearchIndexBuilder;
use crate::cmd::watch::service::shortcodes_compiler::ShortcodesCompiler;
use crate::holder::Holder;
use crate::mcp_server_factory::McpServerFactory;
use crate::poet_error::PoetError;

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
    async fn handle(&self) -> Result<(), PoetError> {
        let ctrlc_notifier = CancellationToken::new();
        let ctrlc_notifier_handler = ctrlc_notifier.clone();

        ctrlc::set_handler(move || {
            ctrlc_notifier_handler.cancel();
        })
        .map_err(PoetError::SetCtrlcHandler)?;

        let ProjectFileWatcher {
            debouncer: _debouncer,
            notifications:
                ProjectFileNotifications {
                    on_author_file_changed,
                    on_content_file_changed,
                    on_esbuild_metafile_changed,
                    on_prompt_file_changed,
                    on_shortcode_file_changed,
                },
        } = watch_project_files(&self.source_directory)?;

        let generated_page_base_path = format!("http://{}/", self.addr);
        let asset_path_renderer = AssetPathRenderer {
            base_path: generated_page_base_path.clone(),
        };
        let build_project_result_holder: Holder<BuildProjectResult> = Holder::default();
        let esbuild_metafile_holder = Holder::default();
        let filesystem_http_route_index_holder = Holder::default();
        let prompt_document_controller_collection_holder = Holder::default();
        let rhai_template_renderer_holder = Holder::default();
        let search_index_reader_holder = Holder::default();
        let source_filesystem = self.source_filesystem();
        let mcp_server = McpServerFactory {
            build_project_result_holder: build_project_result_holder.clone(),
            prompt_document_controller_collection_holder:
                prompt_document_controller_collection_holder.clone(),
            search_index_reader_holder: search_index_reader_holder.clone(),
            server_info: Implementation {
                description: None,
                name: "poet".to_owned(),
                title: Some("Poet".to_owned()),
                version: env!("CARGO_PKG_VERSION").to_owned(),
            },
        }
        .create();

        let mut service_manager = ServiceManager::default();

        service_manager.register_service(Arc::new(EsbuildMetafileReader {
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
            mcp_server: mcp_server.clone(),
        }));

        service_manager.register_service(Arc::new(ProjectBuilder {
            asset_path_renderer: asset_path_renderer.clone(),
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            esbuild_metafile_holder: esbuild_metafile_holder.clone(),
            generate_sitemap: self.sitemap,
            generated_page_base_path,
            on_author_file_changed,
            on_content_file_changed,
            rhai_template_renderer_holder: rhai_template_renderer_holder.clone(),
            source_filesystem: source_filesystem.clone(),
        }));

        service_manager.register_service(Arc::new(PromptDocumentControllerCollectionBuilder {
            asset_path_renderer,
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            esbuild_metafile_holder,
            on_prompt_file_changed,
            prompt_document_controller_collection_holder,
            rhai_template_renderer_holder: rhai_template_renderer_holder.clone(),
            source_filesystem: source_filesystem.clone(),
        }));

        service_manager.register_service(Arc::new(ResourcesListChangedBroadcaster {
            build_project_result_holder: build_project_result_holder.clone(),
            ctrlc_notifier: ctrlc_notifier.clone(),
            session_manager: mcp_server.session_manager,
        }));

        service_manager.register_service(Arc::new(SearchIndexBuilder {
            build_project_result_holder,
            ctrlc_notifier: ctrlc_notifier.clone(),
            search_index_reader_holder,
        }));

        service_manager.register_service(Arc::new(ShortcodesCompiler {
            ctrlc_notifier,
            on_shortcode_file_changed,
            rhai_template_renderer_holder,
            source_filesystem,
        }));

        service_manager.run().await?;

        info!("Poet is shutting down...");

        Ok(())
    }
}
