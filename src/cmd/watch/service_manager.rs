use std::sync::Arc;

use actix_web::rt;
use anyhow::Result;
use log::error;
use tokio::task::JoinSet;

use crate::cmd::watch::service::Service;

#[derive(Default)]
pub struct ServiceManager {
    services: Vec<Arc<dyn Service>>,
}

impl ServiceManager {
    pub fn register_service(&mut self, service: Arc<dyn Service>) {
        self.services.push(service);
    }

    pub async fn run(self) -> Result<()> {
        let mut task_set = JoinSet::new();

        for service in self.services {
            task_set.spawn(rt::spawn(async move {
                if let Err(err) = service.run().await {
                    error!("Service error: {err:#?}");
                }
            }));
        }

        // Stop if any of the tasks stop
        task_set.join_next().await;

        Ok(())
    }
}
