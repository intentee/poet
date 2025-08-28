use std::path::Path as StdPath;

use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::get;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;

use crate::cmd::watch::app_data::AppData;
use crate::cmd::watch::respond_with_file::respond_with_file;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/{path:.*}")]
async fn respond(app_data: Data<AppData>, path: Path<String>) -> Result<HttpResponse> {
    let path_string = path.into_inner();
    let std_path = StdPath::new(&path_string);

    respond_with_file(app_data, std_path, true).await
}
