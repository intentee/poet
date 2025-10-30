use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::get;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;

use crate::cmd::respond_with_generated_page::respond_with_generated_page;
use crate::cmd::serve::app_data::AppData;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/{path:.*}")]
async fn respond(app_data: Data<AppData>, path: Path<String>) -> Result<HttpResponse> {
    respond_with_generated_page(
        app_data.filesystem_http_route_index.clone(),
        path.into_inner(),
    )
}
