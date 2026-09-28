use std::net::TcpStream;

use tungstenite::Message;
use tungstenite::WebSocket;
use tungstenite::stream::MaybeTlsStream;

use crate::poet_process_tests_error::PoetProcessTestsError;

pub fn read_live_reload_until(
    live_reload_socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    expected_fragment: &str,
) -> Result<String, PoetProcessTestsError> {
    loop {
        if let Message::Text(page_contents) = live_reload_socket.read().map_err(Box::new)?
            && page_contents.as_str().contains(expected_fragment)
        {
            return Ok(page_contents.as_str().to_owned());
        }
    }
}
