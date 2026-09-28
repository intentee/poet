use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer as ActixHttpServer;
use actix_web::web::Data;
use async_trait::async_trait;
use poet_mcp::mcp_http_service_factory::McpHttpServiceFactory;
use poet_mcp::mcp_server::McpServer;
use tokio::fs::create_dir_all;
use tokio_util::sync::CancellationToken;

use crate::cmd::HTTP_SERVER_SHUTDOWN_TIMEOUT_SECONDS;
use crate::cmd::MCP_STREAMABLE_HTTP_MOUNT_PATH;
use crate::cmd::STATIC_FILES_PUBLIC_PATH;
use crate::cmd::service::Service;
use crate::cmd::watch::app_data::AppData;
use crate::cmd::watch::http_route;
use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::holder::Holder;
use crate::poet_error::PoetError;

pub struct HttpServer {
    pub address: SocketAddr,
    pub assets_directory: PathBuf,
    pub ctrlc_notifier: CancellationToken,
    pub filesystem_http_route_index_holder: Holder<Arc<FilesystemHttpRouteIndex>>,
    pub mcp_server: McpServer,
}

#[async_trait]
impl Service for HttpServer {
    async fn run(&self) -> Result<(), PoetError> {
        create_dir_all(&self.assets_directory)
            .await
            .map_err(|source| PoetError::CreateAssetsDirectory {
                path: self.assets_directory.clone(),
                source,
            })?;

        let app_data = Data::new(AppData {
            filesystem_http_route_index_holder: self.filesystem_http_route_index_holder.clone(),
        });
        let assets_directory = self.assets_directory.clone();
        let ctrlc_notifier = self.ctrlc_notifier.clone();
        let mcp_server = self.mcp_server.clone();

        let http_server = ActixHttpServer::new(move || {
            App::new()
                .app_data(app_data.clone())
                .service(
                    Files::new(STATIC_FILES_PUBLIC_PATH, assets_directory.clone())
                        .prefer_utf8(true),
                )
                .service(McpHttpServiceFactory {
                    mcp_server: mcp_server.clone(),
                    mount_path: MCP_STREAMABLE_HTTP_MOUNT_PATH.to_owned(),
                })
                .configure(http_route::live_reload::register)
                .configure(http_route::generated_pages::register)
        })
        .bind(self.address)
        .map_err(|source| PoetError::BindHttpServer {
            address: self.address,
            source,
        })?;

        http_server
            .shutdown_signal(async move {
                ctrlc_notifier.cancelled().await;
            })
            .shutdown_timeout(HTTP_SERVER_SHUTDOWN_TIMEOUT_SECONDS)
            .run()
            .await
            .map_err(PoetError::RunHttpServer)
    }
}
