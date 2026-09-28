use std::net::SocketAddr;
use std::net::ToSocketAddrs as _;

use crate::poet_error::PoetError;

pub fn parse_socket_addr(address: &str) -> Result<SocketAddr, PoetError> {
    address
        .to_socket_addrs()
        .map_err(|source| PoetError::ResolveSocketAddress {
            address: address.to_owned(),
            source,
        })
        .and_then(|resolved_addresses| {
            let resolved_addresses: Vec<SocketAddr> = resolved_addresses.collect();

            resolved_addresses
                .iter()
                .find(|resolved_address| resolved_address.is_ipv4())
                .or_else(|| resolved_addresses.first())
                .copied()
                .ok_or(PoetError::SocketAddressUnresolved {
                    address: address.to_owned(),
                })
        })
}
