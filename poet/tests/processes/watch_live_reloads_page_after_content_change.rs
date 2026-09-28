use tungstenite::Message;
use tungstenite::connect;

use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::read_live_reload_until::read_live_reload_until;
use crate::run_poet_server::run_poet_server;

#[test]
fn watch_live_reloads_page_after_content_change() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let address = free_socket_address()?;
    let PoetServerRun {
        exit_status,
        outcome: reloaded_page,
    } = run_poet_server(
        &[
            "watch",
            "--addr",
            &address.to_string(),
            &fixture_site.path_string(),
        ],
        address,
        || -> Result<String, PoetProcessTestsError> {
            let (mut live_reload_socket, _handshake_response) =
                connect(format!("ws://{address}/api/v1/live_reload/")).map_err(Box::new)?;

            read_live_reload_until(&mut live_reload_socket, "Home body.")?;
            live_reload_socket
                .send(Message::text("ignored by the server"))
                .map_err(Box::new)?;
            fixture_site.write(
                "content/index.md",
                "+++\ndescription = \"Home\"\nlayout = \"LayoutPlain\"\ntitle = \"Home\"\n+++\n\nChanged body.\n",
            )?;

            let reloaded_page = read_live_reload_until(&mut live_reload_socket, "Changed body.")?;

            live_reload_socket.close(None).map_err(Box::new)?;

            Ok(reloaded_page)
        },
    )?;

    assert!(exit_status.success());
    assert!(reloaded_page?.contains("Changed body."));

    Ok(())
}
