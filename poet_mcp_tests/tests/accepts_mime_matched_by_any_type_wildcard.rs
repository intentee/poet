use actix_web::http::header::HeaderValue;
use poet_mcp_tests::accepts_all_with_header::accepts_all_with_header;

#[test]
fn accepts_mime_matched_by_any_type_wildcard() {
    assert!(matches!(
        accepts_all_with_header(HeaderValue::from_static("*/*"), &[mime::TEXT_EVENT_STREAM]),
        Ok(())
    ));
}
