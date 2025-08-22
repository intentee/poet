mod app_data;
mod http_route;
mod output_filesystem_holder;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use actix_web::App;
use actix_web::HttpServer;
use actix_web::rt;
use actix_web::web::Data;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use log::error;
use log::info;
use notify::EventKind;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::RecursiveMode;
use tokio::sync::watch;

use self::app_data::AppData;
use self::output_filesystem_holder::OutputFilesystemHolder;
use super::Handler;
use super::value_parser::parse_socket_addr;
use super::value_parser::validate_is_directory;
use crate::build_project::build_project;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;

#[derive(Parser)]
pub struct Watch {
    #[arg(long, default_value = "127.0.0.1:8050", value_parser = parse_socket_addr)]
    addr: SocketAddr,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,
}

#[async_trait]
impl Handler for Watch {
    async fn handle(&self) -> Result<()> {
        let source_filesystem = Storage {
            base_directory: self.source_directory.clone(),
        };
        let output_filesystem_holder: Arc<OutputFilesystemHolder<Memory>> =
            Arc::new(OutputFilesystemHolder::default());
        let (files_changed_tx, mut files_changed_rx) = watch::channel(());

        let mut debouncer = new_debouncer(
            Duration::from_millis(50),
            None,
            move |result: DebounceEventResult| match result {
                Ok(events) => {
                    for event in &events {
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                                info!("Source file change detected");

                                files_changed_tx
                                    .send(())
                                    .expect("Failed to send file change notification");

                                break;
                            }
                            _ => {}
                        }
                    }
                }
                Err(errors) => errors.iter().for_each(|error| error!("{error:?}")),
            },
        )?;

        debouncer.watch(self.source_directory.clone(), RecursiveMode::Recursive)?;

        let output_filesystem_holder_clone = output_filesystem_holder.clone();

        rt::spawn(async move {
            loop {
                match build_project(&source_filesystem).await {
                    Ok(memory_filesystem) => {
                        if let Err(err) = output_filesystem_holder_clone
                            .set_output_filesystem(Arc::new(memory_filesystem))
                        {
                            error!("Failed to set output filesystem: {err}");
                        }
                    }
                    Err(err) => error!("Failed to build project: {err}"),
                }

                if let Err(err) = files_changed_rx.changed().await {
                    error!("Failed to receive file change notification: {err}");
                }
            }
        });

        let app_data = Data::new(AppData {
            output_filesystem_holder,
        });

        HttpServer::new(move || {
            App::new()
                .app_data(app_data.clone())
                .configure(http_route::favicon::register)
                .configure(http_route::static_files::register)
        })
        .bind(self.addr)
        .expect("Unable to bind server to address")
        .run()
        .await?;

        Ok(())
    }
}
