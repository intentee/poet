mod app_data;
mod http_route;
mod output_filesystem_holder;
mod resolve_generated_page_path;
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
use log::error;
use log::info;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use self::app_data::AppData;
use self::output_filesystem_holder::OutputFilesystemHolder;
use self::watch_project_files::WatchProjectHandle;
use self::watch_project_files::watch_project_files;
use super::Handler;
use super::value_parser::parse_socket_addr;
use super::value_parser::validate_is_directory;
use super::value_parser::validate_is_directory_or_create;
use crate::build_project::build_project;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;

#[derive(Parser)]
pub struct Watch {
    #[arg(long, default_value = "127.0.0.1:8050", value_parser = parse_socket_addr)]
    addr: SocketAddr,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,

    #[arg(long, default_value = "static", value_parser = validate_is_directory_or_create)]
    static_files_directory: PathBuf,
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
            mut content_files_changed_rx,
            debouncer: _debouncer,
        } = watch_project_files(self.source_directory.clone())?;

        let output_filesystem_holder: Arc<OutputFilesystemHolder<Memory>> =
            Arc::new(OutputFilesystemHolder::default());
        let output_filesystem_holder_clone = output_filesystem_holder.clone();

        let source_filesystem = Storage {
            base_directory: self.source_directory.clone(),
        };

        let mut task_set = JoinSet::new();

        let ctrlc_notifier_builder = ctrlc_notifier.clone();

        task_set.spawn(rt::spawn(async move {
            loop {
                tokio::select! {
                    _ = content_files_changed_rx.changed() => {
                        match build_project(true, &source_filesystem).await {
                            Ok(memory_filesystem) => {
                                if let Err(err) = output_filesystem_holder_clone
                                    .set_output_filesystem(Arc::new(memory_filesystem))
                                {
                                    error!("Failed to set output filesystem: {err}");
                                }
                            }
                            Err(err) => error!("Failed to build project: {err}"),
                        }
                    }
                    _ = ctrlc_notifier_builder.cancelled() => {
                        break;
                    }
                }
            }
        }));

        let addr = self.addr;
        let ctrlc_notifier_server = ctrlc_notifier.clone();
        let static_files_directory = self.static_files_directory.clone();

        task_set.spawn(rt::spawn(async move {
            let app_data = Data::new(AppData {
                output_filesystem_holder,
            });

            loop {
                if ctrlc_notifier_server.is_cancelled() {
                    break;
                }

                let app_data_clone = app_data.clone();
                let ctrlc_notifier_server_clone = ctrlc_notifier_server.clone();
                let static_files_directory_clone = static_files_directory.clone();

                if let Err(err) = HttpServer::new(move || {
                    App::new()
                        .app_data(app_data_clone.clone())
                        .service(
                            Files::new("/static", static_files_directory_clone.clone())
                                .prefer_utf8(true),
                        )
                        .configure(http_route::favicon::register)
                        .configure(http_route::live_reload::register)
                        .configure(http_route::generated_pages::register)
                })
                .bind(addr)
                .expect("Unable to bind server to address")
                .shutdown_signal(async move { ctrlc_notifier_server_clone.cancelled().await })
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
