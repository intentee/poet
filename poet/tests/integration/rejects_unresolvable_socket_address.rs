use poet::cmd::value_parser::parse_socket_addr::parse_socket_addr;
use poet::poet_error::PoetError;

#[test]
fn rejects_unresolvable_socket_address() {
    assert!(matches!(
        parse_socket_addr("definitely-not-a-socket-address"),
        Err(PoetError::ResolveSocketAddress { address, .. }) if address == "definitely-not-a-socket-address"
    ));
}
