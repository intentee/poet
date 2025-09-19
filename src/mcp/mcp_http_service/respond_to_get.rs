use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::body::BoxBody;

pub struct RespondToGet {}

impl Responder for RespondToGet {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().body("hello, world, get".to_string())
    }
}
