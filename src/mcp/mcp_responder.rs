use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use async_trait::async_trait;
use mime::Mime;

#[async_trait(?Send)]
pub trait McpResponder: Clone {
    fn accepts() -> Vec<Mime>;

    async fn respond_to(&self, req: HttpRequest, payload: Payload)
    -> Result<HttpResponse<BoxBody>>;
}
