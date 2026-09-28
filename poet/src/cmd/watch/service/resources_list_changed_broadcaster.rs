use anyhow::Result;
use async_trait::async_trait;
use poet_mcp::jsonrpc_version::JSONRPC_VERSION;
use poet_mcp::resources_list_changed_notification::ResourcesListChangedNotification;
use poet_mcp::server_to_client_notification::ServerToClientNotification;
use poet_mcp::session_manager::SessionManager;
use tokio_util::sync::CancellationToken;

use crate::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::service::Service;

pub struct ResourcesListChangedBroadcaster {
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
    pub session_manager: SessionManager,
}

impl ResourcesListChangedBroadcaster {
    async fn broadcast_resources_list_changed(&self) {
        self.session_manager
            .broadcast(ServerToClientNotification::ResourcesListChanged(
                ResourcesListChangedNotification {
                    jsonrpc: JSONRPC_VERSION.to_owned(),
                },
            ))
            .await;
    }
}

#[async_trait]
impl Service for ResourcesListChangedBroadcaster {
    async fn run(&self) -> Result<()> {
        loop {
            tokio::select! {
                () = self.build_project_result_holder.update_notifier.notified() => {
                    self.broadcast_resources_list_changed().await;
                }
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
