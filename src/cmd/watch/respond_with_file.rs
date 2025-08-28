use std::path::Path as StdPath;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::web::Data;
use log::error;

use crate::cmd::watch::app_data::AppData;
use crate::filesystem::Filesystem as _;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;

pub async fn respond_with_file(
    app_data: Data<AppData>,
    std_path: &StdPath,
    check_for_index: bool,
) -> Result<HttpResponse> {
    match app_data.output_filesystem_holder.get_output_filesystem() {
        Ok(Some(filesystem)) => match filesystem.read_file_contents(std_path).await {
            Ok(ReadFileContentsResult::Directory) => {
                if check_for_index {
                    Box::pin(respond_with_file(
                        app_data,
                        &std_path.join("index.html"),
                        false,
                    ))
                    .await
                } else {
                    Ok(HttpResponse::NotFound().body(format!(
                        "Found directory, but no index.html file in it: {}",
                        std_path.display()
                    )))
                }
            }
            Ok(ReadFileContentsResult::Found(contents)) => Ok(HttpResponse::Ok()
                .content_type(mime_guess::from_path(std_path).first_or_octet_stream())
                .body(contents)),
            Ok(ReadFileContentsResult::NotFound) => {
                Ok(HttpResponse::NotFound().body("File not found"))
            }
            Err(err) => {
                let msg = format!("Failed to read file contents: {err}");

                error!("{msg}");

                Ok(HttpResponse::InternalServerError().body(msg))
            }
        },
        Ok(None) => Ok(HttpResponse::ServiceUnavailable()
            .body("Server is still starting up, or there are no successful builds yet")),
        Err(err) => {
            let msg = format!("Failed to get output filesystem: {err}");

            error!("{msg}");

            Ok(HttpResponse::InternalServerError().body(msg))
        }
    }
}
