use core::{fmt::Display, str::FromStr};

use crate::sys;

pub struct TcpStream(u32);

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Generic,
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        use embedded_io::ErrorKind;
        match *self {
            Error::Generic => ErrorKind::Other,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl TcpStream {
    pub fn connect(addr: SocketAddr) -> Result<TcpStream> {
        let handle = sys::socket::creat(sys::socket::AF_INET, sys::socket::STREAM, 0);
        let addr = sys::socket::AddrIpv4 {
            family: sys::socket::AF_INET as i16,
            port: addr.1.swap_bytes(),
            ip: addr.0,
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

impl embedded_io::ErrorType for TcpStream {
    type Error = Error;
}

impl embedded_io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> core::prelude::v1::Result<usize, Self::Error> {
        let (count, success) = unsafe { sys::socket::write(self.0, buf.as_ptr(), buf.len()) };
        if success {
            Ok(count)
        } else {
            Err(Error::Generic)
        }
    }

    fn flush(&mut self) -> core::prelude::v1::Result<(), Self::Error> {
        todo!()
    }
}

impl embedded_io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> core::prelude::v1::Result<usize, Self::Error> {
        let (count, success) = unsafe { sys::socket::read(self.0, buf.as_mut_ptr(), buf.len()) };
        if success {
            Ok(count)
        } else {
            Err(Error::Generic)
        }
    }
}

pub struct SocketAddr(u32, u16);

impl SocketAddr {
    pub fn new<A: Into<IpAddr>>(addr: A, port: u16) -> SocketAddr {
        SocketAddr(addr.into().0, port)
    }
}

impl FromStr for SocketAddr {
    type Err = ();
    fn from_str(s: &str) -> core::result::Result<SocketAddr, ()> {
        let Some((host, port)) = s.split_once(':') else {
            return Err(());
        };
        let ip: IpAddr = host.parse()?;
        let port: u16 = port.parse().map_err(|_| ())?;
        Ok(SocketAddr(ip.0, port))
    }
}

impl core::fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{}:{}", IpAddr(self.0), self.1)
    }
}

pub struct IpAddr(u32);

impl From<[u8; 4]> for IpAddr {
    fn from(a: [u8; 4]) -> IpAddr {
        IpAddr((a[0] as u32) | ((a[1] as u32) << 8) | ((a[2] as u32) << 16) | ((a[3] as u32) << 24))
    }
}

impl FromStr for IpAddr {
    type Err = ();
    fn from_str(s: &str) -> core::result::Result<IpAddr, ()> {
        let mut host_name = heapless::Vec::<u8, 128>::new();
        for c in s.chars() {
            _ = host_name.push(c as u8);
        }
        if host_name.push(0).is_err() {
            return Err(());
        }

        unsafe {
            let Some(host_ent) = sys::socket::get_host_by_name(host_name.as_ptr()) else {
                return Err(());
            };

            let host_ent = &*host_ent;
            if host_ent.address_size != 4 {
                return Err(());
            }
            let addr = *host_ent.addresses;
            if addr.is_null() {
                Err(())
            } else {
                Ok(IpAddr(*addr))
            }
        }
    }
}

impl Display for IpAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let a = self.0;
        core::write!(
            f,
            "{}.{}.{}.{}",
            a as u8,
            (a >> 8) as u8,
            (a >> 16) as u8,
            (a >> 24) as u8,
        )
    }
}
