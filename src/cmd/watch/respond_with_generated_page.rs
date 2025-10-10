use std::path::Path as StdPath;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::web::Data;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::cmd::watch::app_data::AppData;
use crate::cmd::watch::resolve_generated_page::resolve_generated_page;
use crate::filesystem::file_entry_stub::FileEntryStub;
use crate::holder::Holder as _;

pub async fn respond_with_generated_page(
    app_data: Data<AppData>,
    std_path: &StdPath,
    check_for_index: bool,
) -> Result<HttpResponse> {
    match app_data.build_project_result_holder.get().await {
        Some(BuildProjectResult {
            memory_filesystem, ..
        }) => match resolve_generated_page(memory_filesystem, std_path, check_for_index).await? {
            Some(FileEntryStub {
                contents,
                relative_path,
            }) => Ok(HttpResponse::Ok()
                .content_type(mime_guess::from_path(relative_path).first_or_octet_stream())
                .body(contents)),
            None => Ok(HttpResponse::NotFound().body("File not found")),
        },
        None => Ok(HttpResponse::ServiceUnavailable()
            .body("Server is still starting up, or there are no successful builds yet")),
    }
}
