use actix_web::HttpResponse;
use actix_web::Result;

use crate::cmd::respond_with_generated_page::respond_with_generated_page;
use crate::filesystem_http_route_index_holder::FilesystemHttpRouteIndexHolder;
use crate::holder::Holder as _;

pub async fn respond_with_generated_page_holder(
    filesystem_http_route_index_holder: FilesystemHttpRouteIndexHolder,
    path: String,
) -> Result<HttpResponse> {
    match filesystem_http_route_index_holder.get().await {
        Some(filesystem_http_route_index) => {
            respond_with_generated_page(filesystem_http_route_index, path)
        }
        None => Ok(HttpResponse::ServiceUnavailable()
            .body("Server is still starting up, or there are no successful builds yet")),
    }
}
