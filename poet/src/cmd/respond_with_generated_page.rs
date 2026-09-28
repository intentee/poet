use actix_web::HttpResponse;
use poet_filesystem::file_entry::FileEntry;

use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;

#[must_use]
pub fn respond_with_generated_page(
    filesystem_http_route_index: &FilesystemHttpRouteIndex,
    route: &str,
) -> HttpResponse {
    match filesystem_http_route_index.file_entry_for_route(route) {
        Some(FileEntry {
            contents,
            relative_path,
            ..
        }) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(relative_path).first_or_octet_stream())
            .body(contents.clone()),
        None => HttpResponse::NotFound().body("File not found"),
    }
}
