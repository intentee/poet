use std::io;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;

pub fn free_socket_address() -> io::Result<SocketAddr> {
    TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?.local_addr()
}
