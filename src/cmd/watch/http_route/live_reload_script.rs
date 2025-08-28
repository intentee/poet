use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/api/v1/live_reload_script.js")]
async fn respond() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/javascript")
        .body(include_str!("../../../../live-reload-script.js")))
}
