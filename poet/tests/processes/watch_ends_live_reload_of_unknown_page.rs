use tungstenite::Error as WebSocketError;
use tungstenite::Message;
use tungstenite::connect;
use tungstenite::protocol::CloseFrame;
use tungstenite::protocol::frame::coding::CloseCode;

use crate::fixture_site::FixtureSite;
use crate::free_socket_address::free_socket_address;
use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::run_poet_server::run_poet_server;
use crate::wait_for_page::wait_for_page;

#[test]
fn watch_ends_live_reload_of_unknown_page() -> Result<(), PoetProcessTestsError> {
    let fixture_site = FixtureSite::create()?;
    let address = free_socket_address()?;
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
        || -> Result<Result<Message, WebSocketError>, PoetProcessTestsError> {
            wait_for_page(address, "/")?;

            let (mut live_reload_socket, _handshake_response) =
                connect(format!("ws://{address}/api/v1/live_reload/missing")).map_err(Box::new)?;

            Ok(live_reload_socket.read())
        },
    )?;

    assert!(exit_status.success());
    assert!(matches!(
        outcome?,
        Ok(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            ..
        })))
    ));

    Ok(())
}
