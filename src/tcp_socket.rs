use utils::*;

use std::mem;
use std::io::Result;
use std::net::{TcpStream, SocketAddr, SocketAddrV4, SocketAddrV6};
// @TODO: Implement ToRawFd, IntoRawFd, FromRawFd on TcpSocket.
use std::os::unix::io::{RawFd, FromRawFd};
use libc;

// From std::sys::unix::net
#[cfg(target_os = "linux")]
use libc::SOCK_CLOEXEC;
#[cfg(not(target_os = "linux"))]
const SOCK_CLOEXEC: libc::c_int = 0;

pub struct TcpSocket(RawFd);

impl TcpSocket {
    pub fn new(ipv: IP) -> Result<TcpSocket> {
        // From std::sys::unix::net Socket::new_raw
        unsafe {
            let fam = ipv.to_af_inet();
            let ty = libc::SOCK_STREAM;

            if cfg!(target_os = "linux") {
                match cvt(libc::socket(fam, ty | SOCK_CLOEXEC, 0)) {
                    Ok(fd) => return Ok(TcpSocket(fd)),
                    Err(ref e) if e.raw_os_error() == Some(libc::EINVAL) => {}
                    Err(e) => return Err(e),
                }
            }

            let socket = TcpSocket(cvt(libc::socket(fam, ty, 0))?);
            socket.set_cloexec()?;
            Ok(socket)
        }
    }

    pub unsafe fn setsockopt(&self,
                             level: libc::c_int,
                             name: libc::c_int,
                             value: *const libc::c_void,
                             option_len: libc::socklen_t)
                             -> Result<libc::c_int> {
        cvt(libc::setsockopt(self.0, level, name, value, option_len))
    }

    pub fn as_tcp_stream(&self) -> TcpStream {
        unsafe { TcpStream::from_raw_fd(self.0) }
    }

    pub fn bind(&self, addr: SocketAddr) -> Result<libc::c_int> {
        unsafe {
            let (mut sockaddr, sockaddr_size) = self.sockaddr(addr);
            cvt(libc::bind(self.0,
                           &mut sockaddr as *mut _ as *mut libc::sockaddr,
                           sockaddr_size))
        }
    }

    pub fn listen(&self) -> Result<libc::c_int> {
        unsafe { cvt(libc::listen(self.0, 128)) }
    }

    pub fn connect(&self, addr: SocketAddr) -> Result<libc::c_int> {
        unsafe {
            let (mut sockaddr, sockaddr_size) = self.sockaddr(addr);
            cvt(libc::connect(self.0,
                              &mut sockaddr as *mut _ as *mut libc::sockaddr,
                              sockaddr_size))
        }
    }

    // From std::sys::unix::net Socket
    #[cfg(not(any(target_env = "newlib", target_os = "solaris", target_os = "emscripten")))]
    fn set_cloexec(&self) -> Result<()> {
        unsafe {
            cvt(libc::ioctl(self.0, libc::FIOCLEX))?;
            Ok(())
        }
    }
    #[cfg(any(target_env = "newlib", target_os = "solaris", target_os = "emscripten"))]
    fn set_cloexec(&self) -> Result<()> {
        unsafe {
            let previous = cvt(libc::fcntl(self.0, libc::F_GETFD))?;
            cvt(libc::fcntl(self.0, libc::F_SETFD, previous | libc::FD_CLOEXEC))?;
            Ok(())
        }
    }

    fn sockaddr(&self, addr: SocketAddr) -> (libc::sockaddr_storage, libc::socklen_t) {
        match addr {
            SocketAddr::V4(v4) => self.sockaddr_in4(v4),
            SocketAddr::V6(v6) => self.sockaddr_in6(v6),
        }
    }

    fn sockaddr_in4(&self, addr: SocketAddrV4) -> (libc::sockaddr_storage, libc::socklen_t) {
        unsafe {
            let mut sockaddr: libc::sockaddr_in = mem::zeroed();

            sockaddr.sin_family = libc::AF_INET as libc::sa_family_t;
            sockaddr.sin_port = addr.port().to_be();
            sockaddr.sin_addr.s_addr = u32::from(*addr.ip()).to_be() as libc::in_addr_t;

            let sockaddr = *(&mut sockaddr as *mut _ as *mut libc::sockaddr_storage);
            (sockaddr, sockaddr_in_size())
        }
    }

    fn sockaddr_in6(&self, addr: SocketAddrV6) -> (libc::sockaddr_storage, libc::socklen_t) {
        unsafe {
            let mut sockaddr: libc::sockaddr_in6 = mem::zeroed();
            sockaddr.sin6_family = libc::AF_INET6 as libc::sa_family_t;
            sockaddr.sin6_port = addr.port().to_be();
            sockaddr.sin6_flowinfo = addr.flowinfo();
            sockaddr.sin6_addr.s6_addr = addr.ip().octets();
            sockaddr.sin6_scope_id = addr.scope_id();

            let sockaddr = *(&mut sockaddr as *mut _ as *mut libc::sockaddr_storage);
            (sockaddr, sockaddr_in6_size())
        }
    }
}
