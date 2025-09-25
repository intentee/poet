use std::sync::Arc;

use actix_web::Result;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorInternalServerError;
use uuid::Uuid;

use crate::mcp::MCP_HEADER_SESSION;
use crate::mcp::session::Session;
use crate::mcp::session_storage::SessionStorage;

fn generate_session_id() -> String {
    format!("poet-{}", Uuid::new_v4())
}

#[derive(Clone)]
pub struct SessionManager {
    pub session_storage: Arc<dyn SessionStorage>,
}

impl SessionManager {
    pub async fn restore_session(&self, req: &ServiceRequest) -> Result<Option<Session>> {
        match req.headers().get(MCP_HEADER_SESSION) {
            Some(session_id) => {
                self.session_storage
                    .read(session_id.to_str().map_err(ErrorInternalServerError)?)
                    .await
            }
            None => Ok(None),
        }
    }

    pub async fn start_new_session(&self) -> Result<Session> {
        let session = Session {
            session_id: generate_session_id(),
        };

        self.session_storage
            .store_new_session(session.clone())
            .await?;

        Ok(session)
    }

    #[inline]
    pub async fn terminate_session(&self, session: Session) -> Result<()> {
        self.session_storage.terminate_session(session).await
    }
}
