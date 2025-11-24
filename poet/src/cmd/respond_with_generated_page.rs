use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Result;

use crate::filesystem::file_entry::FileEntry;
use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;

pub fn respond_with_generated_page(
    filesystem_http_route_index: Arc<FilesystemHttpRouteIndex>,
    path: String,
) -> Result<HttpResponse> {
    match filesystem_http_route_index.get_file_entry_for_path(&path) {
        Some(FileEntry {
            contents,
            relative_path,
            ..
        }) => Ok(HttpResponse::Ok()
            .content_type(mime_guess::from_path(relative_path).first_or_octet_stream())
            .body(contents)),
        None => Ok(HttpResponse::NotFound().body("File not found")),
    }
}
