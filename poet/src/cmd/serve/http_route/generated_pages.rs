use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;

use crate::cmd::respond_with_generated_page::respond_with_generated_page;
use crate::cmd::serve::app_data::AppData;

#[get("/{path:.*}")]
async fn respond(app_data: Data<AppData>, path: Path<String>) -> HttpResponse {
    respond_with_generated_page(&app_data.filesystem_http_route_index, &path)
}

pub fn register(service_config: &mut web::ServiceConfig) {
    service_config.service(respond);
}
