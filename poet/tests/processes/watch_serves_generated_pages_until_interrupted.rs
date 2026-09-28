use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::http_get::http_get;
use crate::http_response::HttpResponse;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::run_poet_server::run_poet_server;
use crate::wait_for_page::wait_for_page;

#[test]
fn watch_serves_generated_pages_until_interrupted() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let address = free_socket_address()?;
    let PoetServerRun {
        exit_status,
        outcome: http_responses,
    } = run_poet_server(
        &[
            "watch",
            "--addr",
            &address.to_string(),
            &fixture_site.path_string(),
        ],
        address,
        || [wait_for_page(address, "/"), http_get(address, "/missing")],
    )?;

    assert!(exit_status.success());
    assert!(matches!(
        &http_responses,
        [
            Ok(HttpResponse { body: home_body, .. }),
            Ok(HttpResponse { status: 404, .. }),
        ] if home_body.contains("Home body.")
    ));

    Ok(())
}
