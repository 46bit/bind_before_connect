// Heavily based upon
//   https://github.com/rust-lang/rust/blob/master/src/libstd/sys/unix/net.rs

extern crate libc;

mod utils;
mod tcp_socket;
// These aren't really separate modules. The separate files just make it more organised.
pub use utils::*;
pub use tcp_socket::*;

use std::io::Result;
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};

pub fn bind_before_connect<A: ToSocketAddrs>(source_addr: A, dest_addr: A) -> Result<TcpStream> {
    // 1. Find a binding source socket.
    // 2. Find a connecting destination socket.

    let socket = each_addr(source_addr, try_socket_and_bind)?;

    // Set SO_REUSEADDR. Generally a good idea when setting our own source addresses.
    unsafe {
        socket.setsockopt(libc::SOL_SOCKET,
                        libc::SO_REUSEADDR,
                        &(1 as libc::c_int) as *const _ as *const libc::c_void,
                        c_int_size())?;
    }

    // Use FromRawFd to return the socket as a std::io::TcpStream. A TcpStream has everything
    // people need from here on out, and is easily pluggable into all sorts of libraries.
    match each_addr_with_param(dest_addr, &socket, try_connect) {
        Ok(_) => Ok(socket.as_tcp_stream()),
        Err(e) => Err(e),
    }
}

// Iterate over each resolution of the source address, and find one that
// socket() and bind() successfully.
fn try_socket_and_bind(source_addr: &SocketAddr) -> Result<TcpSocket> {
    // @TODO: Are we certain that we only ever connect IPv4-IPv4 and IPv6-IPv6.
    //        If the two are mixed, then setting AF_INET/AF_INET6 based upon the
    //        source address may be unwise.
    let socket = TcpSocket::new(IP::from_socket_addr(*source_addr))?;
    socket.bind(*source_addr)?;
    Ok(socket)
}

// Iterate over each resolution of the destination address, and find one that
// the socket created above can connect() to.
fn try_connect(dest_addr: &SocketAddr, socket: &TcpSocket) -> Result<libc::c_int> {
    socket.connect(*dest_addr)
}
