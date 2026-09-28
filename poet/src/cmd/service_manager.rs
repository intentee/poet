use std::sync::Arc;

use tokio::task::JoinSet;

use crate::cmd::service::Service;
use crate::poet_error::PoetError;

#[derive(Default)]
pub struct ServiceManager {
    services: Vec<Arc<dyn Service>>,
}

impl ServiceManager {
    pub fn register_service(&mut self, service: Arc<dyn Service>) {
        self.services.push(service);
    }

    pub async fn run(self) -> Result<(), PoetError> {
        let mut task_set = JoinSet::new();

        for service in self.services {
            task_set.spawn_local(async move { service.run().await });
        }

        task_set
            .join_next()
            .await
            .map_or(Ok(()), |finished_service| {
                finished_service
                    .map_err(PoetError::ServiceTask)
                    .and_then(|service_result| service_result)
            })
    }
}
