use std::sync::Arc;

use actix_web::HttpResponse;

use crate::cmd::respond_with_generated_page::respond_with_generated_page;
use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;

#[must_use]
pub fn respond_with_generated_page_holder(
    filesystem_http_route_index_holder: &Holder<Arc<FilesystemHttpRouteIndex>>,
    route: &str,
) -> HttpResponse {
    match filesystem_http_route_index_holder.get() {
        HolderState::Ready(filesystem_http_route_index) => {
            respond_with_generated_page(&filesystem_http_route_index, route)
        }
        HolderState::NotReady => HttpResponse::ServiceUnavailable()
            .body(PoetError::BuildProjectResultNotReady.to_string()),
    }
}
