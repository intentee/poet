use std::convert::Infallible;
use std::path::Path as StdPath;
use std::time::Duration;

use actix_web::Error;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web_lab::sse;
use log::error;
use log::warn;

use crate::cmd::watch::app_data::AppData;
use crate::cmd::watch::resolve_generated_page_path::resolve_generated_page_path;
use crate::filesystem::file_entry::FileEntry;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/api/v1/live_reload/{path:.*}")]
async fn respond(app_data: Data<AppData>, path: Path<String>) -> Result<impl Responder, Error> {
    let event_stream = async_stream::stream! {
        let path_string = path.into_inner();
        let std_path = StdPath::new(&path_string);

        loop {
            app_data.output_filesystem_holder.update_notifier.notified().await;

            match app_data.output_filesystem_holder.get_output_filesystem() {
                Ok(Some(filesystem)) => {
                    match resolve_generated_page_path(filesystem, std_path, true).await {
                        Ok(Some(FileEntry {
                            contents,
                            relative_path: _,
                        })) => {
                            yield (Ok::<_, Infallible>(sse::Event::Data(sse::Data::new(contents))));
                        },
                        Ok(None) => {
                            warn!("Unable to get file info for live reload: {path_string}");
                        },
                        Err(err) => error!("Unable to resolve generated file path: {err}"),
                    }
                },
                Ok(None) => warn!("Server is still starting up, or there are no successful builds yet"),
                Err(err) => error!("Failed to get buffered requests snapshot: {err}"),
            }
        }
    };

    Ok(sse::Sse::from_stream(event_stream).with_keep_alive(Duration::from_secs(10)))
}
