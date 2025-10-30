use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::get;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;

use crate::cmd::watch::app_data::AppData;
use crate::cmd::watch::respond_with_generated_page_holder::respond_with_generated_page_holder;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/{path:.*}")]
async fn respond(app_data: Data<AppData>, path: Path<String>) -> Result<HttpResponse> {
    respond_with_generated_page_holder(
        app_data.filesystem_http_route_index_holder.clone(),
        path.into_inner(),
    )
    .await
}
