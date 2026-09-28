use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::get;
use actix_web::rt;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::Payload;
use actix_ws::CloseCode;
use actix_ws::CloseReason;
use actix_ws::Message;
use actix_ws::Session;
use futures_util::StreamExt as _;
use log::debug;
use log::warn;
use poet_content::generated_file::GeneratedFile;

use crate::cmd::watch::app_data::AppData;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;
use crate::report_poet_error::report_poet_error;

async fn close_live_reload_session(session: Session, close_reason: Option<CloseReason>) {
    session
        .close(close_reason)
        .await
        .map_err(PoetError::CloseLiveReloadSession)
        .unwrap_or_else(report_poet_error);
}

#[get("/api/v1/live_reload/{path:.*}")]
async fn respond(
    app_data: Data<AppData>,
    path: Path<String>,
    request: HttpRequest,
    payload: Payload,
) -> Result<impl Responder, Error> {
    let (response, mut session, mut message_stream) = actix_ws::handle(&request, payload)?;
    let mut filesystem_http_route_index_updates =
        app_data.filesystem_http_route_index_holder.subscribe();

    rt::spawn(async move {
        let route = path.into_inner();

        loop {
            match app_data.filesystem_http_route_index_holder.get() {
                HolderState::Ready(filesystem_http_route_index) => {
                    let Some(GeneratedFile { contents, .. }) =
                        filesystem_http_route_index.generated_file_for_route(&route)
                    else {
                        warn!("Unable to get file info for live reload: {route}");

                        close_live_reload_session(
                            session,
                            Some(CloseReason {
                                code: CloseCode::Normal,
                                description: Some(format!(
                                    "There is no page to reload at '{route}'"
                                )),
                            }),
                        )
                        .await;

                        return;
                    };

                    session
                        .text(contents.clone())
                        .await
                        .map_err(PoetError::SendLiveReloadPage)
                        .unwrap_or_else(report_poet_error);
                }
                HolderState::NotReady => warn!("{}", PoetError::BuildProjectResultNotReady),
            }

            tokio::select! {
                message = message_stream.next() => {
                    match message {
                        None | Some(Ok(Message::Close(_))) => {
                            debug!("Closing live reload session");
                            close_live_reload_session(session, None).await;

                            return;
                        },
                        ignored_message => {
                            warn!("Live reload socket message was ignored: {ignored_message:?}");
                        }
                    }
                },
                Ok(()) = filesystem_http_route_index_updates.changed() => {}
            }
        }
    });

    Ok(response)
}

pub fn register(service_config: &mut web::ServiceConfig) {
    service_config.service(respond);
}
