use actix_web::http::StatusCode;
use poet::cmd::respond_with_generated_page::respond_with_generated_page;
use poet::filesystem_http_route_index::FilesystemHttpRouteIndex;
use poet_filesystem::memory::Memory;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn responds_not_found_for_unknown_route() -> Result<(), PoetTestsError> {
    assert_eq!(
        respond_with_generated_page(
            &FilesystemHttpRouteIndex::from_memory(&Memory::default())?,
            "missing"
        )
        .status(),
        StatusCode::NOT_FOUND
    );

    Ok(())
}
