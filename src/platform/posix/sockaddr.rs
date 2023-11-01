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

use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ptr;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use libc::c_uchar;
#[cfg(any(target_os = "linux", target_os = "android"))]
use libc::c_ushort;

use libc::{AF_INET as _AF_INET, AF_INET6, sockaddr_in6};
use libc::{in_addr, sockaddr, sockaddr_in};

use crate::error::*;

/// A wrapper for `sockaddr_in`.
#[derive(Copy, Clone)]
pub union SockAddr {
    sa4: sockaddr_in,
    sa6: sockaddr_in6,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
const AF_INET: c_ushort = _AF_INET as c_ushort;

#[cfg(any(target_os = "macos", target_os = "ios"))]
const AF_INET: c_uchar = _AF_INET as c_uchar;

impl SockAddr {
    /// Create a new `SockAddr` from a generic `sockaddr`.
    pub fn new(value: &sockaddr) -> Result<Self> {
        if value.sa_family != AF_INET {
            return Err(Error::InvalidAddress);
        }

        unsafe { Self::unchecked(value) }
    }

    /// # Safety
    ///  Create a new `SockAddr` and not check the source.
    pub unsafe fn unchecked(value: &sockaddr) -> Result<Self> {
        Ok(SockAddr{
            sa6: ptr::read(value as *const _ as *const _)
        })
    }

    /// # Safety
    /// Get a generic pointer to the `SockAddr`.
    pub unsafe fn as_ptr(&self) -> *const sockaddr {
        &self.sa6 as *const _ as *const sockaddr
    }
}

impl From<IpAddr> for SockAddr {
    fn from(ip: IpAddr) -> SockAddr {
        // let octets = ip.octets();
        // let mut addr = unsafe { mem::zeroed::<sockaddr_in>() };
        //
        // addr.sin_family = AF_INET;
        // addr.sin_port = 0;
        // addr.sin_addr = in_addr {
        //     s_addr: u32::from_ne_bytes(octets),
        // };
        //
        // SockAddr(addr)

        let mut result : Self = unsafe { std::mem::zeroed() };
        match ip {
            IpAddr::V4(addr) => {
                let sa4 = unsafe { &mut result.sa4 };

                #[cfg(any(
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "haiku",
                target_os = "hermit",
                target_os = "ios",
                target_os = "macos",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "vxworks",
                target_os = "watchos",
                ))]
                {
                    sa4.sin_len = std::mem::size_of::<sockaddr_in>() as u8;
                }

                sa4.sin_family = AF_INET.try_into().unwrap();
                sa4.sin_port = u16::to_be(0);

                let raw_ip = u32::to_be((addr).into());
                #[cfg(not(target_os = "windows"))]
                {
                    sa4.sin_addr.s_addr = raw_ip;
                }
                #[cfg(target_os = "windows")]
                unsafe {
                    *sa4.sin_addr.S_un.S_addr_mut() = raw_ip;
                }
            },
            IpAddr::V6(addr) => {
                let sa6 = unsafe { &mut result.sa6 };

                #[cfg(any(
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "haiku",
                target_os = "hermit",
                target_os = "ios",
                target_os = "macos",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "vxworks",
                target_os = "watchos",
                ))]
                {
                    sa6.sin6_len = std::mem::size_of::<sockaddr_in6>() as u8;
                }

                sa6.sin6_family = AF_INET6.try_into().unwrap();
                sa6.sin6_port = u16::to_be(0);
                //sa6.sin6_flowinfo = addr.flowinfo();

                let raw_ip = u128::to_be_bytes((addr).into());
                #[cfg(not(target_os = "windows"))]
                {
                    sa6.sin6_addr.s6_addr = raw_ip;
                    //sa6.sin6_scope_id = addr.scope_id();
                }
                #[cfg(target_os = "windows")]
                unsafe {
                    *sa6.sin6_addr.u.Byte_mut() = raw_ip;
                    *sa6.u.sin6_scope_id_mut() = addr.scope_id();
                }
            }
        }
        result
    }
}

impl From<SockAddr> for Ipv4Addr {
    fn from(addr: SockAddr) -> Ipv4Addr {
        let ip = unsafe { addr.sa4 }.sin_addr.s_addr;

        Ipv4Addr::from(u32::from_be(ip))
    }
}

impl From<SockAddr> for Ipv6Addr {
    fn from(addr: SockAddr) -> Ipv6Addr {
        let ip = unsafe { addr.sa6 }.sin6_addr.s6_addr;

        Ipv6Addr::from(u128::from_be_bytes(ip))
    }
}

impl From<SockAddr> for IpAddr {
    fn from(addr: SockAddr) -> IpAddr {
        let ip = unsafe { addr.sa6 }.sin6_addr.s6_addr;

        IpAddr::from(Ipv6Addr::from(u128::from_be_bytes(ip)))
    }
}

impl From<SockAddr> for sockaddr {
    fn from(addr: SockAddr) -> sockaddr {
        unsafe { mem::transmute(addr.sa4) }
    }
}

impl From<SockAddr> for sockaddr_in6 {
    fn from(addr: SockAddr) -> sockaddr_in6 {
        unsafe { addr.sa6 }
    }
}
