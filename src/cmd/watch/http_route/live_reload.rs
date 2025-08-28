use std::convert::Infallible;
use std::path::Path as StdPath;
use std::time::Duration;

use actix_web::Error;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use actix_web::web::Path;
use actix_web_lab::sse;
use log::error;
use log::info;
use log::warn;

use crate::cmd::watch::app_data::AppData;
use crate::filesystem::Filesystem as _;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/api/v1/live_reload/{path:.*}")]
async fn respond(
    app_data: web::Data<AppData>,
    path: Path<String>,
) -> Result<impl Responder, Error> {
    info!("Live reload request for path: {path}");

    let event_stream = async_stream::stream! {
        let path_string = path.into_inner();
        let std_path = StdPath::new(&path_string);

        loop {
            app_data.output_filesystem_holder.update_notifier.notified().await;

            match app_data.output_filesystem_holder.get_output_filesystem() {
                Ok(Some(filesystem)) => {
                    match filesystem.read_file_contents(std_path).await {
                        Ok(ReadFileContentsResult::Found(contents)) => {
                            yield (Ok::<_, Infallible>(sse::Event::Data(sse::Data::new(contents))));
                        },
                        _ => {
                            warn!("Unable to get file info for live reload: {path_string}");
                        }
                    }
                },
                Ok(None) => warn!("Server is still starting up, or there are no successful builds yet"),
                Err(err) => error!("Failed to get buffered requests snapshot: {err}"),
            }
        }
    };

    Ok(sse::Sse::from_stream(event_stream).with_keep_alive(Duration::from_secs(10)))
}
