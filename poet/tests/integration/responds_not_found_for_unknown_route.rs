use actix_web::http::StatusCode;
use poet::cmd::respond_with_generated_page::respond_with_generated_page;
use poet::filesystem_http_route_index::FilesystemHttpRouteIndex;

#[test]
fn responds_not_found_for_unknown_route() {
    assert_eq!(
        respond_with_generated_page(
            &FilesystemHttpRouteIndex::from_generated_files(&[]),
            "missing"
        )
        .status(),
        StatusCode::NOT_FOUND
    );
}
