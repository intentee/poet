use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::http_get::http_get;
use crate::http_response::HttpResponse;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::run_poet_server::run_poet_server;

#[test]
fn serve_serves_generated_pages_until_interrupted() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let address = free_socket_address()?;
    let PoetServerRun {
        exit_status,
        outcome: http_responses,
    } = run_poet_server(
        &[
            "serve",
            "--addr",
            &address.to_string(),
            "--app-name",
            "fixture",
            "--public-path",
            "/",
            &fixture_site.path_string(),
        ],
        address,
        || {
            [
                http_get(address, "/"),
                http_get(address, "/docs/page/"),
                http_get(address, "/missing"),
            ]
        },
    )?;

    assert!(exit_status.success());
    assert!(matches!(
        &http_responses,
        [
            Ok(HttpResponse { status: 200, body: home_body }),
            Ok(HttpResponse { status: 200, body: page_body }),
            Ok(HttpResponse { status: 404, .. }),
        ] if home_body.contains("Home body.") && page_body.contains("Page body.")
    ));

    Ok(())
}
