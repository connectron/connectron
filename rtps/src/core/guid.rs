use std::{mem, default::Default, net::IpAddr, ops::{Deref, DerefMut}};

use core::VendorId;
use libc;
use pnet;

/// Type used to hold globally-unique RTPS-entity identifiers.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Guid {
    prefix: GuidPrefix,
    entity_id: EntityId,
}

impl Guid {
    pub fn unknown() -> Self {
        Self {
            prefix: GuidPrefix::unknown(),
            entity_id: EntityId::unknown(),
        }
    }

    pub fn new(participant_id: i32) -> Self {
        Self {
            prefix: GuidPrefix::automatic(participant_id),
            entity_id: EntityId::unknown(),
        }
    }

    pub fn set_entity_of_participant(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::participant(),
        }
    }

    pub fn set_entity_of_sedp_builtin_topic_writer(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_topic_writer(),
        }
    }

    pub fn set_entity_of_sedp_builtin_topic_reader(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_topic_reader(),
        }
    }

    pub fn set_entity_of_sedp_builtin_publications_writer(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_publications_writer(),
        }
    }

    pub fn set_entity_of_sedp_builtin_publications_reader(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_publications_reader(),
        }
    }

    pub fn set_entity_of_sedp_builtin_subscriptions_writer(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_subscriptions_writer(),
        }
    }

    pub fn set_entity_of_sedp_builtin_subscriptions_reader(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_subscriptions_reader(),
        }
    }

    pub fn set_entity_of_sedp_builtin_participant_writer(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_participant_writer(),
        }
    }

    pub fn set_entity_of_sedp_builtin_participant_reader(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::sedp_builtin_participant_reader(),
        }
    }

    pub fn set_entity_of_p2p_builtin_participant_message_writer(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::p2p_builtin_participant_message_writer(),
        }
    }

    pub fn set_entity_of_p2p_builtin_participant_message_reader(self) -> Self {
        Self {
            prefix: self.prefix,
            entity_id: EntityId::p2p_builtin_participant_message_reader(),
        }
    }
}

/// Type used to hold the prefix of the globally-unique RTPS-entity identifiers.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuidPrefix([u8; 12]);

impl GuidPrefix {
    pub fn automatic(participant_id: i32) -> Self {
        let vendor_id = VendorId::default();
        let addr = match pnet::datalink::interfaces()
            .iter()
            .filter(|i| !i.is_loopback())
            .next()
            .and_then(|i| i.ips.iter().next().map(|ip| ip.ip()))
        {
            Some(IpAddr::V4(addr)) => addr.octets()
                .iter()
                .rev()
                .take(2)
                .cloned()
                .collect::<Vec<_>>(),
            Some(IpAddr::V6(addr)) => addr.octets()
                .iter()
                .rev()
                .take(2)
                .cloned()
                .collect::<Vec<_>>(),
            _ => vec![1u8, 127],
        };
        let pid = unsafe { libc::getpid() } as i32;
        let pid = unsafe { mem::transmute::<i32, [u8; 4]>(pid.to_be()) };
        let id = unsafe { mem::transmute::<i32, [u8; 4]>(participant_id.to_be()) };
        let inner = [
            vendor_id[0],
            vendor_id[1],
            addr[1],
            addr[0],
            pid[0],
            pid[1],
            pid[2],
            pid[3],
            id[0],
            id[1],
            id[2],
            id[3],
        ];

        GuidPrefix(inner)
    }

    pub fn unknown() -> Self {
        GuidPrefix([0; 12])
    }
}

impl Default for GuidPrefix {
    fn default() -> Self {
        Self::unknown()
    }
}

impl Deref for GuidPrefix {
    type Target = [u8; 12];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuidPrefix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Type used to hold the suffix part of the globally-unique RTPS-entity identifiers.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EntityId([u8; 4]);

impl EntityId {
    #[inline]
    pub fn unknown() -> Self {
        EntityId([0; 4])
    }

    #[inline]
    pub fn participant() -> Self {
        EntityId([0, 0, 1, 0xc1])
    }

    #[inline]
    pub fn sedp_builtin_topic_writer() -> Self {
        EntityId([0, 0, 2, 0xc2])
    }

    #[inline]
    pub fn sedp_builtin_topic_reader() -> Self {
        EntityId([0, 0, 2, 0xc7])
    }

    #[inline]
    pub fn sedp_builtin_publications_writer() -> Self {
        EntityId([0, 0, 3, 0xc2])
    }

    #[inline]
    pub fn sedp_builtin_publications_reader() -> Self {
        EntityId([0, 0, 3, 0xc7])
    }

    #[inline]
    pub fn sedp_builtin_subscriptions_writer() -> Self {
        EntityId([0, 0, 4, 0xc2])
    }

    #[inline]
    pub fn sedp_builtin_subscriptions_reader() -> Self {
        EntityId([0, 0, 4, 0xc7])
    }

    #[inline]
    pub fn sedp_builtin_participant_writer() -> Self {
        EntityId([0, 1, 0, 0xc2])
    }

    #[inline]
    pub fn sedp_builtin_participant_reader() -> Self {
        EntityId([0, 1, 0, 0xc7])
    }

    #[inline]
    pub fn p2p_builtin_participant_message_writer() -> Self {
        EntityId([0, 2, 0, 0xc2])
    }

    #[inline]
    pub fn p2p_builtin_participant_message_reader() -> Self {
        EntityId([0, 2, 0, 0xc7])
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::unknown()
    }
}

impl Deref for EntityId {
    type Target = [u8; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn guid() {
        use super::{EntityId, Guid, GuidPrefix};

        assert_eq!(
            Guid {
                prefix: GuidPrefix::unknown(),
                entity_id: EntityId::unknown(),
            },
            Guid::unknown()
        );
    }

    #[test]
    fn guid_prefix() {
        use libc;

        use super::GuidPrefix;
        use core::VendorId;

        assert_eq!([0; 12], *GuidPrefix::unknown());

        let id = 0x12345678i32;
        let pid = unsafe { libc::getpid() } as i32;
        let prefix = GuidPrefix::automatic(id);
        assert_eq!(prefix[0], VendorId::default()[0]);
        assert_eq!(prefix[1], VendorId::default()[1]);
        assert_eq!(prefix[4], pid.to_be() as u8);
        assert_eq!(prefix[5], (pid.to_be() >> 8) as u8);
        assert_eq!(prefix[6], (pid.to_be() >> 16) as u8);
        assert_eq!(prefix[7], (pid.to_be() >> 24) as u8);
        assert_eq!(prefix[8], id.to_be() as u8);
        assert_eq!(prefix[9], (id.to_be() >> 8) as u8);
        assert_eq!(prefix[10], (id.to_be() >> 16) as u8);
        assert_eq!(prefix[11], (id.to_be() >> 24) as u8);
    }

    #[test]
    fn entity_id() {
        use super::EntityId;

        assert_eq!([0; 4], *EntityId::unknown());
        assert_eq!(*EntityId::unknown(), *EntityId::default());
        assert_eq!([0, 0, 1, 0xc1], *EntityId::participant());
        assert_eq!([0, 0, 2, 0xc2], *EntityId::sedp_builtin_topic_writer());
        assert_eq!([0, 0, 2, 0xc7], *EntityId::sedp_builtin_topic_reader());
        assert_eq!(
            [0, 0, 3, 0xc2],
            *EntityId::sedp_builtin_publications_writer()
        );
        assert_eq!(
            [0, 0, 3, 0xc7],
            *EntityId::sedp_builtin_publications_reader()
        );
        assert_eq!(
            [0, 0, 4, 0xc2],
            *EntityId::sedp_builtin_subscriptions_writer()
        );
        assert_eq!(
            [0, 0, 4, 0xc7],
            *EntityId::sedp_builtin_subscriptions_reader()
        );
        assert_eq!(
            [0, 1, 0, 0xc2],
            *EntityId::sedp_builtin_participant_writer()
        );
        assert_eq!(
            [0, 1, 0, 0xc7],
            *EntityId::sedp_builtin_participant_reader()
        );
        assert_eq!(
            [0, 2, 0, 0xc2],
            *EntityId::p2p_builtin_participant_message_writer()
        );
        assert_eq!(
            [0, 2, 0, 0xc7],
            *EntityId::p2p_builtin_participant_message_reader()
        );
    }
}
