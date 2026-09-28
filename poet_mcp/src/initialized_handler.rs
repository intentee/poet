use actix_web::HttpResponse;
use actix_web::body::BoxBody;

use crate::initialized_notification::InitializedNotification;
use crate::log_level::LogLevel;
use crate::mcp_header_session::MCP_HEADER_SESSION;
use crate::message_notification_params::MessageNotificationParams;
use crate::session::Session;

pub struct InitializedHandler;

impl InitializedHandler {
    pub async fn handle(
        self,
        _: InitializedNotification,
        session: &Session,
    ) -> HttpResponse<BoxBody> {
        session
            .notifier
            .log_message(MessageNotificationParams {
                data: "Initialization handshake is successfully completed".to_owned(),
                level: LogLevel::Debug,
            })
            .await;

        HttpResponse::Accepted()
            .insert_header((MCP_HEADER_SESSION, session.id.clone()))
            .finish()
    }
}
