use async_trait::async_trait;
use poet_content::build_project_result::BuildProjectResult;
use poet_mcp::jsonrpc_version::JSONRPC_VERSION;
use poet_mcp::resources_list_changed_notification::ResourcesListChangedNotification;
use poet_mcp::server_to_client_notification::ServerToClientNotification;
use poet_mcp::session_manager::SessionManager;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::holder::Holder;
use crate::poet_error::PoetError;

pub struct ResourcesListChangedBroadcaster {
    pub build_project_result_holder: Holder<BuildProjectResult>,
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
    async fn run(&self) -> Result<(), PoetError> {
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
