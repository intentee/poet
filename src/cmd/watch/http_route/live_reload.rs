use std::path::Path as StdPath;

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

use crate::build_project::build_project_result::BuildProjectResult;
use crate::cmd::watch::app_data::AppData;
use crate::cmd::watch::resolve_generated_page::resolve_generated_page;
use crate::filesystem::file_entry::FileEntry;
use crate::holder::Holder as _;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/api/v1/live_reload/{path:.*}")]
async fn respond(
    app_data: Data<AppData>,
    path: Path<String>,
    req: HttpRequest,
    stream: Payload,
) -> Result<impl Responder, Error> {
    let (res, mut session, mut stream) = actix_ws::handle(&req, stream)?;

    rt::spawn(async move {
        let path_string = path.into_inner();
        let std_path = StdPath::new(&path_string);

        loop {
            match app_data.build_project_result_holder.get().await {
                Some(BuildProjectResult {
                    esbuild_metafile: _,
                    markdown_document_reference_collection: _,
                    memory_filesystem,
                }) => match resolve_generated_page(memory_filesystem, std_path, true).await {
                    Ok(Some(FileEntry {
                        contents,
                        relative_path: _,
                    })) => {
                        if let Err(err) = session.text(contents).await {
                            debug!("Unable to send live reload notification: {err}");

                            return;
                        }
                    }
                    Ok(None) => {
                        warn!("Unable to get file info for live reload: {path_string}");
                        return;
                    }
                    Err(err) => {
                        error!("Unable to resolve generated file path: {err}");
                        return;
                    }
                },
                None => {
                    warn!("Server is still starting up, or there are no successful builds yet")
                }
            }

            tokio::select! {
                msg = stream.next() => {
                    match msg {
                        None | Some(Ok(Message::Close(_))) => {
                            debug!("Closing live reload session");

                            if let Err(err) = session.close(None).await {
                                error!("Error while closing the session: {err}");
                            }

                            return;
                        },
                        _ => {
                            warn!("Live reload socket message was ignored: {msg:?}");
                        }
                    }
                },
                _ = app_data.build_project_result_holder.update_notifier.notified() => {}
            }
        }
    });

    Ok(res)
}
