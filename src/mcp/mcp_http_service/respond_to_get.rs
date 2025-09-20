use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::body::BoxBody;

use crate::mcp::accepts_all::Conclusion;
use crate::mcp::accepts_all::accepts_all;

pub struct RespondToGet {}

impl Responder for RespondToGet {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match accepts_all(req, vec![mime::TEXT_EVENT_STREAM]) {
            Conclusion::AllAcceptable => HttpResponse::Ok().body("hello, world, get".to_string()),
            Conclusion::NotAllAcceptable => HttpResponse::NotAcceptable().into(),
            Conclusion::ErrorParsingHeader(err) => HttpResponse::InternalServerError()
                .body(format!("{err}"))
                .into(),
        }
    }
}
