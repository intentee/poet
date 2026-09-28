use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::get;
use actix_web::rt;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::web::Payload;
use actix_ws::Message;
use futures_util::StreamExt as _;
use log::debug;
use log::error;
use log::warn;
use poet_filesystem::file_entry::FileEntry;

use crate::cmd::watch::app_data::AppData;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;

pub fn register(service_config: &mut web::ServiceConfig) {
    service_config.service(respond);
}

#[get("/api/v1/live_reload/{path:.*}")]
async fn respond(
    app_data: Data<AppData>,
    path: Path<String>,
    request: HttpRequest,
    payload: Payload,
) -> Result<impl Responder, Error> {
    let (response, mut session, mut message_stream) = actix_ws::handle(&request, payload)?;

    rt::spawn(async move {
        let route = path.into_inner();

        loop {
            match app_data.filesystem_http_route_index_holder.get() {
                HolderState::Ready(filesystem_http_route_index) => {
                    let Some(FileEntry { contents, .. }) =
                        filesystem_http_route_index.file_entry_for_route(&route)
                    else {
                        warn!("Unable to get file info for live reload: {route}");

                        return;
                    };

                    if let Err(closed_session) = session.text(contents.clone()).await {
                        debug!("Unable to send live reload notification: {closed_session}");

                        return;
                    }
                }
                HolderState::NotReady => warn!("{}", PoetError::BuildProjectResultNotReady),
            }

            tokio::select! {
                message = message_stream.next() => {
                    match message {
                        None | Some(Ok(Message::Close(_))) => {
                            debug!("Closing live reload session");

                            if let Err(close_error) = session.close(None).await {
                                error!("Error while closing the session: {close_error}");
                            }

                            return;
                        },
                        ignored_message => {
                            warn!("Live reload socket message was ignored: {ignored_message:?}");
                        }
                    }
                },
                () = app_data.filesystem_http_route_index_holder.update_notifier.notified() => {}
            }
        }
    });

    Ok(response)
}
