use actix_web::HttpResponse;
use poet_content::generated_file::GeneratedFile;

use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;

#[must_use]
pub fn respond_with_generated_page(
    filesystem_http_route_index: &FilesystemHttpRouteIndex,
    route: &str,
) -> HttpResponse {
    match filesystem_http_route_index.generated_file_for_route(route) {
        Some(GeneratedFile {
            contents,
            relative_path,
            ..
        }) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(relative_path).first_or_octet_stream())
            .body(contents.clone()),
        None => HttpResponse::NotFound().body("File not found"),
    }
}
