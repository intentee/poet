use std::sync::Arc;

use anyhow::Result;
use tokio::task::JoinSet;

use crate::cmd::service::Service;

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
            task_set.spawn_local(async move { service.run().await });
        }

        match task_set.join_next().await {
            Some(finished_service) => finished_service?,
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use anyhow::anyhow;
    use async_trait::async_trait;

    use super::ServiceManager;
    use crate::cmd::service::Service;

    struct FailingService;

    #[async_trait]
    impl Service for FailingService {
        async fn run(&self) -> Result<()> {
            Err(anyhow!("service failed"))
        }
    }

    #[actix_web::test]
    async fn returns_error_of_failed_service() {
        let mut service_manager = ServiceManager::default();

        service_manager.register_service(Arc::new(FailingService));

        assert!(service_manager.run().await.is_err());
    }
}
