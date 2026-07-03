use std::net::SocketAddr;
use std::net::ToSocketAddrs;

use anyhow::Result;
use anyhow::anyhow;

fn resolve_socket_addr(s: &str) -> Result<SocketAddr> {
    let addrs: Vec<SocketAddr> = s.to_socket_addrs()?.collect();

    for addr in &addrs {
        if addr.is_ipv4() {
            return Ok(*addr);
        }
    }

    for addr in addrs {
        if addr.is_ipv6() {
            return Ok(addr);
        }
    }

    Err(anyhow!("Failed to resolve socket address"))
}

pub fn parse_socket_addr(arg: &str) -> Result<SocketAddr> {
    match arg.parse() {
        Ok(socketaddr) => Ok(socketaddr),
        Err(_) => Ok(resolve_socket_addr(arg)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_literal_ipv4_socket_address() -> Result<()> {
        assert_eq!(
            parse_socket_addr("127.0.0.1:8080")?,
            "127.0.0.1:8080".parse::<SocketAddr>()?
        );

        Ok(())
    }

    #[test]
    fn resolve_prefers_ipv4_address() -> Result<()> {
        assert!(resolve_socket_addr("127.0.0.1:8080")?.is_ipv4());

        Ok(())
    }

    #[test]
    fn resolve_returns_ipv6_when_no_ipv4_present() -> Result<()> {
        assert!(resolve_socket_addr("[::1]:8080")?.is_ipv6());

        Ok(())
    }

    #[test]
    fn errors_on_malformed_socket_address() {
        assert!(parse_socket_addr("definitely-not-a-socket-address").is_err());
    }
}
