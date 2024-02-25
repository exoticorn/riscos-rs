use crate::sys;

pub struct TcpStream(u32);

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Generic,
}

pub type Result<T> = core::result::Result<T, Error>;

impl TcpStream {
    pub fn connect(addr: SocketAddr) -> Result<TcpStream> {
        let handle = sys::socket::creat(sys::socket::AF_INET, sys::socket::STREAM, 0);
        let addr = sys::socket::AddrIpv4 {
            family: sys::socket::AF_INET as i16,
            port: addr.1.swap_bytes(),
            ip: addr.0.swap_bytes(),
            pad0: 0,
            pad1: 0,
        };
        let success = unsafe {
            sys::socket::connect(
                handle,
                core::mem::transmute(&addr),
                core::mem::size_of_val(&addr) as u32,
            )
        };
        if success {
            Ok(TcpStream(handle))
        } else {
            sys::socket::close(handle);
            Err(Error::Generic)
        }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        sys::socket::close(self.0)
    }
}

pub struct SocketAddr(u32, u16);

impl SocketAddr {
    pub fn new<A: Into<IpAddr>>(addr: A, port: u16) -> SocketAddr {
        SocketAddr(addr.into().0, port)
    }
}

pub struct IpAddr(u32);

impl From<[u8; 4]> for IpAddr {
    fn from(a: [u8; 4]) -> IpAddr {
        IpAddr(((a[0] as u32) << 24) | ((a[1] as u32) << 16) | ((a[2] as u32) << 8) | a[3] as u32)
    }
}
