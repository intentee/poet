use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Service {
    async fn run(&self) -> Result<()>;
}
