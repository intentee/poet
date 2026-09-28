use tungstenite::connect;

use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::http_get::http_get;
use crate::http_response::HttpResponse;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::read_live_reload_until::read_live_reload_until;
use crate::run_poet_server::run_poet_server;

const VALID_HOME: &str =
    "+++\ndescription = \"Home\"\nlayout = \"LayoutPlain\"\ntitle = \"Home\"\n+++\n\nFixed body.\n";

#[test]
fn watch_recovers_after_fixing_invalid_content() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let address = free_socket_address()?;

    fixture_site.write(
        "content/index.md",
        "+++\ndescription = \"Home\"\nlayout = \"Missing\"\ntitle = \"Home\"\n+++\n",
    )?;

    let PoetServerRun {
        exit_status,
        outcome,
    } = run_poet_server(
        &[
            "watch",
            "--addr",
            &address.to_string(),
            &fixture_site.path_string(),
        ],
        address,
        || -> Result<HttpResponse, PoetProcessTestsError> {
            let (mut live_reload_socket, _handshake_response) =
                connect(format!("ws://{address}/api/v1/live_reload/")).map_err(Box::new)?;
            let unavailable_response = http_get(address, "/")?;

            fixture_site.write("content/index.md", VALID_HOME)?;
            read_live_reload_until(&mut live_reload_socket, "Fixed body.")?;

            Ok(unavailable_response)
        },
    )?;

    assert!(exit_status.success());
    assert!(matches!(outcome, Ok(HttpResponse { status: 503, .. })));

    Ok(())
}
