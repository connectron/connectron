mod error;
mod guid;

pub use self::error::{Error, ErrorKind, Result};
pub use self::guid::{EntityId, Guid, GuidPrefix};

use std::{cmp::Ordering, ops::{Deref, DerefMut}};

/// Type used to represent the version of the RTPS protocol.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ProtocolVersion {
    major: u8,
    minor: u8,
}

impl ProtocolVersion {
    pub fn current() -> Self {
        Self { major: 2, minor: 2 }
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl Ord for ProtocolVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major == other.major {
            self.minor.cmp(&other.minor)
        } else {
            self.major.cmp(&other.major)
        }
    }
}

impl PartialOrd for ProtocolVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Type used to represent the vendor of the service implementing the RTPS protocol.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VendorId([u8; 2]);

impl VendorId {
    pub fn unknown() -> Self {
        VendorId([0; 2])
    }
}

impl Default for VendorId {
    fn default() -> Self {
        // dummy value
        VendorId([0xcf, 0xff])
    }
}

impl Deref for VendorId {
    type Target = [u8; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VendorId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn protocol_version() {
        use super::ProtocolVersion;

        assert_eq!(ProtocolVersion::default(), ProtocolVersion::current());
        assert!(ProtocolVersion { major: 2, minor: 0 } < ProtocolVersion { major: 2, minor: 2 });
        assert!(ProtocolVersion { major: 1, minor: 2 } < ProtocolVersion { major: 2, minor: 0 });
    }

    #[test]
    fn vendor_id() {
        use super::VendorId;

        assert_eq!([0; 2], *VendorId::unknown());
        assert_eq!([0xcf, 0xff], *VendorId::default());
    }
}
