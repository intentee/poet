mod app_data;
mod http_route;
mod output_filesystem_holder;
mod resolve_generated_page;
mod respond_with_generated_page;
mod watch_project_files;

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
use tokio::fs::create_dir_all;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use self::app_data::AppData;
use self::output_filesystem_holder::OutputFilesystemHolder;
use self::watch_project_files::WatchProjectHandle;
use self::watch_project_files::watch_project_files;
use super::Handler;
use super::value_parser::validate_is_directory;
use crate::build_project::build_project;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;
use crate::poet_config::PoetConfig;
use crate::read_poet_config_file::read_poet_config_file;

#[derive(Parser)]
pub struct Watch {
    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,
}

#[async_trait]
impl Handler for Watch {
    async fn handle(&self) -> Result<()> {
        let ctrlc_notifier = CancellationToken::new();
        let ctrlc_notifier_handler = ctrlc_notifier.clone();

        ctrlc::set_handler(move || {
            ctrlc_notifier_handler.cancel();
        })?;

        let poet_config_path = self.source_directory.join("poet.toml").canonicalize()?;

        let WatchProjectHandle {
            debouncer: _debouncer,
            on_content_file_changed,
            on_poet_config_file_changed,
        } = watch_project_files(poet_config_path.clone(), self.source_directory.clone())?;

        let output_filesystem_holder: Arc<OutputFilesystemHolder<Memory>> =
            Arc::new(OutputFilesystemHolder::default());
        let output_filesystem_holder_clone = output_filesystem_holder.clone();

        let source_filesystem = Storage {
            base_directory: self.source_directory.clone(),
        };

        let mut task_set = JoinSet::new();

        let ctrlc_notifier_builder = ctrlc_notifier.clone();

        task_set.spawn(rt::spawn(async move {
            let do_build_project = async || match build_project(true, &source_filesystem).await {
                Ok(memory_filesystem) => {
                    if let Err(err) = output_filesystem_holder_clone
                        .set_output_filesystem(Arc::new(memory_filesystem))
                        .await
                    {
                        error!("Failed to set output filesystem: {err}");
                    }
                }
                Err(err) => error!("Failed to build project: {err}"),
            };

            do_build_project().await;

            loop {
                tokio::select! {
                    _ = on_content_file_changed.notified() => do_build_project().await,
                    _ = ctrlc_notifier_builder.cancelled() => {
                        break;
                    }
                }
            }
        }));

        let ctrlc_notifier_server = ctrlc_notifier.clone();

        task_set.spawn(rt::spawn(async move {
            let app_data = Data::new(AppData {
                output_filesystem_holder,
            });

            loop {
                if ctrlc_notifier_server.is_cancelled() {
                    break;
                }

                let PoetConfig {
                    static_files_directory,
                    static_files_public_path,
                    watch_server_addr,
                } = match read_poet_config_file(&poet_config_path).await {
                    Ok(poet_config) => poet_config,
                    Err(err) => {
                        error!(
                            "Unable to parse config file {}: {err}",
                            poet_config_path.display()
                        );

                        on_poet_config_file_changed.notified().await;

                        continue;
                    }
                };

                if !static_files_directory.exists()
                    && let Err(err) = create_dir_all(&static_files_directory).await
                {
                    error!(
                        "Unable to create static files directory '{}': {err}",
                        static_files_directory.display()
                    );
                }

                let app_data_clone = app_data.clone();
                let ctrlc_notifier_server_clone = ctrlc_notifier_server.clone();
                let on_poet_config_file_changed_clone = on_poet_config_file_changed.clone();
                let static_files_directory_clone = static_files_directory.clone();

                if let Err(err) = HttpServer::new(move || {
                    App::new()
                        .app_data(app_data_clone.clone())
                        .service(
                            Files::new(
                                &static_files_public_path,
                                static_files_directory_clone.clone(),
                            )
                            .prefer_utf8(true),
                        )
                        .configure(http_route::favicon::register)
                        .configure(http_route::live_reload::register)
                        .configure(http_route::generated_pages::register)
                })
                .bind(watch_server_addr)
                .expect("Unable to bind server to address")
                .shutdown_signal(async move {
                    tokio::select! {
                        _ = ctrlc_notifier_server_clone.cancelled() => {}
                        _ = on_poet_config_file_changed_clone.notified() => {}
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
