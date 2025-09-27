mod app_data;
mod http_route;
mod output_filesystem_holder;
mod resolve_generated_page;
mod respond_with_generated_page;
mod watch_project_files;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::rt;
use actix_web::web::Data;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use log::debug;
use log::error;
use log::info;
use tokio::fs::create_dir_all;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use self::app_data::AppData;
use self::output_filesystem_holder::OutputFilesystemHolder;
use self::watch_project_files::WatchProjectHandle;
use self::watch_project_files::watch_project_files;
use super::Handler;
use super::value_parser::parse_socket_addr;
use super::value_parser::validate_is_directory;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::build_project;
use crate::cmd::builds_project::BuildsProject;
use crate::compile_shortcodes::compile_shortcodes;
use crate::filesystem::memory::Memory;
use crate::holder::Holder as _;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::mcp_http_service_factory::McpHttpServiceFactory;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session_manager::SessionManager;
use crate::mcp::session_storage::memory::Memory as MemorySessionStorage;
use crate::rhai_template_renderer_holder::RhaiTemplateRendererHolder;

const STATIC_FILES_PUBLIC_PATH: &str = "assets";

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

#[async_trait]
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

        let assets_directory = self.assets_directory();
        let output_filesystem_holder: Arc<OutputFilesystemHolder<Memory>> =
            Arc::new(OutputFilesystemHolder::default());
        let output_filesystem_holder_clone = output_filesystem_holder.clone();
        let rhai_template_renderer_holder: RhaiTemplateRendererHolder = Default::default();
        let source_filesystem = self.source_filesystem();

        let resource_list_aggregate: Arc<ResourceListAggregate> = Arc::new(vec![].into());

        let mut task_set = JoinSet::new();

        let ctrlc_notifier_rhai = ctrlc_notifier.clone();
        let rhai_template_renderer_holder_rhai = rhai_template_renderer_holder.clone();
        let source_filesystem_rhai = source_filesystem.clone();

        task_set.spawn(rt::spawn(async move {
            let do_compile_shortcodes = async || {
                let rhai_template_renderer =
                    match compile_shortcodes(source_filesystem_rhai.clone()).await {
                        Ok(rhai_template_renderer) => rhai_template_renderer,
                        Err(err) => {
                            error!("Failed to compile shortcodes: {err}");

                            return;
                        }
                    };

                rhai_template_renderer_holder_rhai
                    .set(Some(rhai_template_renderer))
                    .await;
            };

            loop {
                do_compile_shortcodes().await;

                tokio::select! {
                    _ = on_shortcode_file_changed.notified() => {},
                    _ = ctrlc_notifier_rhai.cancelled() => {
                        break;
                    },
                }
            }
        }));

        let addr_builder = self.addr;
        let ctrlc_notifier_builder = ctrlc_notifier.clone();
        let rhai_template_renderer_holder_builder = rhai_template_renderer_holder.clone();
        let source_filesystem_builder = source_filesystem.clone();

        task_set.spawn(rt::spawn(async move {
            let do_build_project = async || {
                let rhai_template_renderer = match rhai_template_renderer_holder_builder.get().await {
                    Some(rhai_template_renderer) => rhai_template_renderer,
                    None => {
                        debug!("Rhai components are not compiled yet. Skipping build");

                        return;
                    },
                };

                let base_path = format!("http://{}/", addr_builder);

                match build_project(
                    AssetPathRenderer {
                        base_path: base_path.clone(),
                    },
                    base_path,
                    true,
                    rhai_template_renderer,
                    source_filesystem_builder.clone(),
                ).await {
                    Ok(build_project_result) => {
                        output_filesystem_holder_clone
                            .set(Some(build_project_result.memory_filesystem.clone()))
                            .await;

                        info!("Build successful");
                    }
                    Err(err) => error!("Failed to build project: {err:#}"),
                }
            };

            loop {
                tokio::select! {
                    _ = rhai_template_renderer_holder_builder.update_notifier.notified() => do_build_project().await,
                    _ = on_content_file_changed.notified() => do_build_project().await,
                    _ = ctrlc_notifier_builder.cancelled() => {
                        break;
                    },
                }
            }
        }));

        let addr_server = self.addr;
        let ctrlc_notifier_server = ctrlc_notifier.clone();
        let resource_list_aggregate_server = resource_list_aggregate.clone();

        task_set.spawn(rt::spawn(async move {
            let app_data = Data::new(AppData {
                output_filesystem_holder,
            });

            loop {
                if ctrlc_notifier_server.is_cancelled() {
                    break;
                }

                if !assets_directory.exists()
                    && let Err(err) = create_dir_all(&assets_directory).await
                {
                    error!(
                        "Unable to create static files directory '{}': {err}",
                        assets_directory.display()
                    );
                }

                let app_data_clone = app_data.clone();
                let assets_directory_clone = assets_directory.clone();
                let ctrlc_notifier_server_clone = ctrlc_notifier_server.clone();
                let resource_list_aggregate_clone = resource_list_aggregate_server.clone();
                let server_info = Implementation {
                    name: "poet".to_string(),
                    title: Some("Poet".to_string()),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                };
                let session_manager = SessionManager {
                    session_storage: Arc::new(MemorySessionStorage::new()),
                };

                if let Err(err) = HttpServer::new(move || {
                    App::new()
                        .app_data(app_data_clone.clone())
                        .service(
                            Files::new(STATIC_FILES_PUBLIC_PATH, assets_directory_clone.clone())
                                .prefer_utf8(true),
                        )
                        .service(McpHttpServiceFactory {
                            mount_path: "/mcp/streamable".to_string(),
                            resource_list_aggregate: resource_list_aggregate_clone.clone(),
                            server_info: server_info.clone(),
                            session_manager: session_manager.clone(),
                        })
                        .configure(http_route::live_reload::register)
                        .configure(http_route::generated_pages::register)
                })
                .bind(addr_server)
                .expect("Unable to bind server to address")
                .shutdown_signal(async move {
                    tokio::select! {
                        _ = ctrlc_notifier_server_clone.cancelled() => {}
                    }
                })
                .shutdown_timeout(1)
                .run()
                .await
                {
                    error!("Unable to start watch server: {err}");
                }
            }
        }));

        // Stop if any of the tasks stop
        task_set.join_next().await;

        info!("Poet is shutting down...");

        Ok(())
    }
}
