use async_trait::async_trait;

use crate::poet_error::PoetError;

#[async_trait]
pub trait Service {
    async fn run(&self) -> Result<(), PoetError>;
}
