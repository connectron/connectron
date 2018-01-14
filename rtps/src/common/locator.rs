use std::net::{IpAddr, SocketAddr};

/// Type used to represent the addressing information needed to send a message
/// to an RTPS Endpoint using one of the supported transports.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Locator {
    kind: i32,
    port: u32,
    address: [u8; 16],
}

impl Locator {
    pub fn new(kind: LocatorKind, socket_addr: SocketAddr) -> Self {
        let kind = match kind {
            LocatorKind::Invalid => -1,
            LocatorKind::Reserved => 0,
            LocatorKind::Udp => match socket_addr {
                SocketAddr::V4(_) => 1,
                SocketAddr::V6(_) => 2,
            },
        };
        Self {
            kind,
            port: socket_addr.port() as _,
            address: match socket_addr.ip() {
                IpAddr::V4(addr) => addr.to_ipv6_compatible().octets(),
                IpAddr::V6(addr) => addr.octets(),
            },
        }
    }

    pub fn invalid() -> Self {
        Self::new(LocatorKind::Invalid, "[::]:0".parse().unwrap())
    }

    pub fn kind(&self) -> LocatorKind {
        match self.kind {
            0 => LocatorKind::Reserved,
            1 | 2 => LocatorKind::Udp,
            _ => LocatorKind::Invalid,
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.address.into(), self.port as _)
    }
}

impl Default for Locator {
    fn default() -> Self {
        Self::new(LocatorKind::Udp, "[::]:0".parse().unwrap())
    }
}

/// Type used to identify the transport that receives the message.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LocatorKind {
    Invalid,
    Reserved,
    Udp,
}

#[cfg(test)]
mod tests {
    #[test]
    fn locator() {
        use std::net::SocketAddr;

        use super::{Locator, LocatorKind};

        assert_eq!(LocatorKind::Invalid, Locator::invalid().kind());
        assert_eq!(
            "[::]:0".parse::<SocketAddr>().unwrap(),
            Locator::invalid().socket_addr()
        );
    }
}
