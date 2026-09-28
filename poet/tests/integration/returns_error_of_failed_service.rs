use std::sync::Arc;

use async_trait::async_trait;
use poet::cmd::service::Service;
use poet::cmd::service_manager::ServiceManager;
use poet::poet_error::PoetError;

struct UnavailableProjectService;

#[async_trait]
impl Service for UnavailableProjectService {
    async fn run(&self) -> Result<(), PoetError> {
        Err(PoetError::BuildProjectResultNotReady)
    }
}

#[actix_web::test]
async fn returns_error_of_failed_service() {
    let mut service_manager = ServiceManager::default();

    service_manager.register_service(Arc::new(UnavailableProjectService));

    assert!(matches!(
        service_manager.run().await,
        Err(PoetError::BuildProjectResultNotReady)
    ));
}
