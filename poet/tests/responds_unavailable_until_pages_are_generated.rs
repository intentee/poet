use actix_web::http::StatusCode;
use poet::cmd::respond_with_generated_page_holder::respond_with_generated_page_holder;
use poet::holder::Holder;

#[test]
fn responds_unavailable_until_pages_are_generated() {
    assert_eq!(
        respond_with_generated_page_holder(&Holder::default(), "").status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
}
