use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::body::BoxBody;

pub struct RespondToPost {}

impl Responder for RespondToPost {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().body("hello, world, post".to_string())
    }
}
