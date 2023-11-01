//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV6};
use std::net::{SocketAddr, SocketAddrV4};

use crate::error::*;

/// Helper trait to convert things into IPv4 addresses.
#[allow(clippy::wrong_self_convention)]
pub trait IntoAddress {
    /// Convert the type to an `Ipv4Addr`.
    fn into_address(&self) -> Result<IpAddr>;
}

impl IntoAddress for u32 {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::from(Ipv4Addr::new(
            ((*self) & 0xff) as u8,
            ((*self >> 8) & 0xff) as u8,
            ((*self >> 16) & 0xff) as u8,
            ((*self >> 24) & 0xff) as u8,
        )))
    }
}

impl IntoAddress for i32 {
    fn into_address(&self) -> Result<IpAddr> {
        (*self as u32).into_address()
    }
}

impl IntoAddress for (u8, u8, u8, u8) {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::from(Ipv4Addr::new(self.0, self.1, self.2, self.3)))
    }
}

impl IntoAddress for str {
    fn into_address(&self) -> Result<IpAddr> {
        self.parse().map_err(|_| Error::InvalidAddress)
    }
}

impl<'a> IntoAddress for &'a str {
    fn into_address(&self) -> Result<IpAddr> {
        (*self).into_address()
    }
}

impl IntoAddress for String {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}

impl<'a> IntoAddress for &'a String {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}

impl IntoAddress for IpAddr {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(*self)
    }
}

impl IntoAddress for Ipv4Addr {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::from(*self))
    }
}

impl IntoAddress for Ipv6Addr {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::from(*self))
    }
}

impl<'a> IntoAddress for &'a Ipv4Addr {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}

impl<'a> IntoAddress for &'a Ipv6Addr {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}


impl<'a> IntoAddress for &'a IpAddr {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}

impl IntoAddress for SocketAddrV4 {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::from(*self.ip()))
    }
}

impl<'a> IntoAddress for &'a SocketAddrV4 {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}

impl IntoAddress for SocketAddrV6 {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::from(*self.ip()))
    }
}

impl<'a> IntoAddress for &'a SocketAddrV6 {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}

impl IntoAddress for SocketAddr {
    fn into_address(&self) -> Result<IpAddr> {
        Ok(IpAddr::from(self.ip()))
    }
}

impl<'a> IntoAddress for &'a SocketAddr {
    fn into_address(&self) -> Result<IpAddr> {
        (**self).into_address()
    }
}
