use poet::cmd::value_parser::parse_socket_addr::parse_socket_addr;

use crate::poet_tests_error::PoetTestsError;

#[test]
fn falls_back_to_ipv6_socket_address() -> Result<(), PoetTestsError> {
    assert!(parse_socket_addr("[::1]:8080").map_err(Box::new)?.is_ipv6());

    Ok(())
}
