use actix_web::Result;
use async_trait::async_trait;

use crate::mcp::session::Session;

pub mod memory;

#[async_trait]
pub trait SessionStorage: Send + Sync {
    async fn read(&self, session_id: &str) -> Result<Option<Session>>;

    async fn store_new_session(&self, session: Session) -> Result<()>;

    async fn terminate_session(&self, session: Session) -> Result<()>;

    async fn update_session(&self, session: Session) -> Result<()>;
}
