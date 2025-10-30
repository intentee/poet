use anyhow::Result;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Handler {
    async fn handle(&self) -> Result<()>;
}
