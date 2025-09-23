use actix_web::Result;
use async_trait::async_trait;
use dashmap::DashMap;

use crate::mcp::session::Session;
use crate::mcp::session_storage::SessionStorage;

#[derive(Clone)]
pub struct Memory {
    sessions: DashMap<String, Session>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
        }
    }
}

#[async_trait]
impl SessionStorage for Memory {
    async fn read(&self, session_id: &str) -> Result<Option<Session>> {
        Ok(self.sessions.get(session_id).map(|session| session.clone()))
    }

    async fn store_new_session(&self, session: Session) -> Result<()> {
        self.sessions.insert(session.session_id.clone(), session);

        Ok(())
    }

    async fn terminate_session(&self, Session { session_id }: Session) -> Result<()> {
        self.sessions.remove(&session_id);

        Ok(())
    }
}
