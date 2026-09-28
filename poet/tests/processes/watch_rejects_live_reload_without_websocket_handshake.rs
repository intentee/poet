use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::http_get::http_get;
use crate::http_response::HttpResponse;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::run_poet_server::run_poet_server;

#[test]
fn watch_rejects_live_reload_without_websocket_handshake() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let address = free_socket_address()?;
    let PoetServerRun {
        exit_status,
        outcome: http_response,
    } = run_poet_server(
        &[
            "watch",
            "--addr",
            &address.to_string(),
            &fixture_site.path_string(),
        ],
        address,
        || http_get(address, "/api/v1/live_reload/"),
    )?;

    assert!(exit_status.success());
    assert!(matches!(
        http_response,
        Ok(HttpResponse { status: 400, .. })
    ));

    Ok(())
}
