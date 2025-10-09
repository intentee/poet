use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer as ActixHttpServer;
use actix_web::web::Data;
use anyhow::Result;
use async_trait::async_trait;
use log::error;
use tokio::fs::create_dir_all;
use tokio_util::sync::CancellationToken;

use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::watch::app_data::AppData;
use crate::cmd::watch::http_route;
use crate::cmd::watch::service::Service;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::mcp::mcp_http_service_factory::McpHttpServiceFactory;
use crate::mcp::resource_list_aggregate::ResourceListAggregate;
use crate::mcp::session_manager::SessionManager;

const STATIC_FILES_PUBLIC_PATH: &str = "assets";

pub struct HttpServer {
    pub addr: SocketAddr,
    pub assets_directory: PathBuf,
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
    pub resource_list_aggregate: Arc<ResourceListAggregate>,
}

#[async_trait]
impl Service for HttpServer {
    async fn run(self: Arc<Self>) -> Result<()> {
        loop {
            if self.ctrlc_notifier.is_cancelled() {
                break;
            }

            if !self.assets_directory.exists()
                && let Err(err) = create_dir_all(&self.assets_directory).await
            {
                error!(
                    "Unable to create static files directory '{}': {err}",
                    self.assets_directory.display()
                );
            }

            let app_data = Data::new(AppData {
                build_project_result_holder: self.build_project_result_holder.clone(),
            });
            let assets_directory = self.assets_directory.clone();
            let ctrlc_notifier = self.ctrlc_notifier.clone();
            let resource_list_aggregate = self.resource_list_aggregate.clone();
            let server_info = Implementation {
                name: "poet".to_string(),
                title: Some("Poet".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
            };
            let session_manager = SessionManager {
                session_storage: Arc::new(Default::default()),
            };

            if let Err(err) = ActixHttpServer::new(move || {
                App::new()
                    .app_data(app_data.clone())
                    .service(
                        Files::new(STATIC_FILES_PUBLIC_PATH, assets_directory.clone())
                            .prefer_utf8(true),
                    )
                    .service(McpHttpServiceFactory {
                        mount_path: "/mcp/streamable".to_string(),
                        resource_list_aggregate: resource_list_aggregate.clone(),
                        server_info: server_info.clone(),
                        session_manager: session_manager.clone(),
                    })
                    .configure(http_route::live_reload::register)
                    .configure(http_route::generated_pages::register)
            })
            .bind(self.addr)
            .expect("Unable to bind server to address")
            .shutdown_signal(async move {
                ctrlc_notifier.cancelled().await;
            })
            .shutdown_timeout(1)
            .run()
            .await
            {
                error!("Unable to start watch server: {err}");
            }
        }

        Ok(())
    }
}
