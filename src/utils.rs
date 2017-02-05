use std::io::{Result, Error, ErrorKind};
use std::net;
use std::mem;
use std::net::{SocketAddr, ToSocketAddrs};
use libc;

pub enum IP {
    V4,
    V6,
}

impl IP {
    pub fn to_af_inet(&self) -> i32 {
        match *self {
            IP::V4 => libc::AF_INET,
            IP::V6 => libc::AF_INET6,
        }
    }

    pub fn from_ip_addr(ip_addr: net::IpAddr) -> IP {
        match ip_addr {
            net::IpAddr::V4(_) => IP::V4,
            net::IpAddr::V6(_) => IP::V6,
        }
    }

    pub fn from_socket_addr(socket_addr: net::SocketAddr) -> IP {
        match socket_addr {
            net::SocketAddr::V4(_) => IP::V4,
            net::SocketAddr::V6(_) => IP::V6,
        }
    }
}

// From std::sys::unix::mod IsMinusOne, cvt
pub fn cvt(t: libc::c_int) -> Result<libc::c_int> {
    if t == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(t)
    }
}

pub fn c_int_size() -> libc::socklen_t {
    mem::size_of::<libc::c_int>() as libc::socklen_t
}

pub fn sockaddr_in_size() -> libc::socklen_t {
    mem::size_of::<libc::sockaddr_in>() as libc::socklen_t
}

pub fn sockaddr_in6_size() -> libc::socklen_t {
    mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t
}

// From std::net::mod each_addr
pub fn each_addr<A: ToSocketAddrs, F, T>(addr: A, mut f: F) -> Result<T>
    where F: FnMut(&SocketAddr) -> Result<T>
{
    let mut last_err = None;
    for addr in addr.to_socket_addrs()? {
        match f(&addr) {
            Ok(l) => return Ok(l),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| {
        Error::new(ErrorKind::InvalidInput,
                   "could not resolve to any addresses")
    }))
}

// From std::net::mod each_addr
pub fn each_addr_with_param<A: ToSocketAddrs, B, F, T>(addr: A, param: &B, mut f: F) -> Result<T>
    where F: FnMut(&SocketAddr, &B) -> Result<T>
{
    let mut last_err = None;
    for addr in addr.to_socket_addrs()? {
        match f(&addr, param) {
            Ok(l) => return Ok(l),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| {
        Error::new(ErrorKind::InvalidInput,
                   "could not resolve to any addresses")
    }))
}
