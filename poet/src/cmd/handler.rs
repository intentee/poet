use async_trait::async_trait;

use crate::poet_error::PoetError;

#[async_trait(?Send)]
pub trait Handler {
    async fn handle(&self) -> Result<(), PoetError>;
}
