use std::net::Ipv4Addr;
use std::net::SocketAddr;

use poet::cmd::value_parser::parse_socket_addr::parse_socket_addr;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn parses_literal_socket_address() -> Result<(), PoetTestsError> {
    assert_eq!(
        parse_socket_addr("127.0.0.1:8080").map_err(Box::new)?,
        SocketAddr::from((Ipv4Addr::LOCALHOST, 8080))
    );

    Ok(())
}
