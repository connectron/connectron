use std::{default::Default, ops::{Add, AddAssign, Sub, SubAssign}};

/// Type used to hold sequence numbers.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SequenceNumber {
    high: i32,
    low: u32,
}

impl SequenceNumber {
    #[inline]
    pub fn new(high: i32, low: u32) -> Self {
        SequenceNumber { high, low }
    }

    #[inline]
    pub fn unknown() -> Self {
        SequenceNumber::new(-1, 0)
    }

    #[inline]
    pub fn zero() -> Self {
        SequenceNumber::new(0, 0)
    }

    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        if self == Self::unknown() || rhs == Self::unknown() {
            return None;
        }

        let (low, overflowed) = self.low.overflowing_add(rhs.low);
        if let Some(high) = self.high
            .checked_add(rhs.high)
            .and_then(|h| h.checked_add(if overflowed { 1 } else { 0 }))
        {
            Some(Self::new(high, low))
        } else {
            None
        }
    }

    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        if self == Self::unknown() || rhs == Self::unknown() {
            return None;
        }

        let (low, overflowed) = self.low.overflowing_sub(rhs.low);
        if let Some(high) = self.high
            .checked_sub(rhs.high)
            .and_then(|h| h.checked_sub(if overflowed { 1 } else { 0 }))
        {
            if high < 0 {
                None
            } else {
                Some(Self::new(high, low))
            }
        } else {
            None
        }
    }
}

impl Add for SequenceNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect("invalid values or overflow when adding sequence numbers")
    }
}

impl AddAssign for SequenceNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for SequenceNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect("invalid values or overflow when subtracting sequence numbers")
    }
}

impl SubAssign for SequenceNumber {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Default for SequenceNumber {
    fn default() -> Self {
        SequenceNumber::new(0, 1)
    }
}

impl From<i64> for SequenceNumber {
    fn from(v: i64) -> Self {
        Self {
            high: (v >> 32) as _,
            low: (v & 0xffffffff) as _,
        }
    }
}

impl Into<i64> for SequenceNumber {
    fn into(self) -> i64 {
        (self.high as i64) << 32 + self.low as i64
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceNumber;

    #[test]
    fn sequence_number() {
        assert_eq!(SequenceNumber::new(0, 1), SequenceNumber::default());
        assert_eq!(SequenceNumber::new(0, 0), SequenceNumber::zero());
        assert_eq!(-1i64 << 32, SequenceNumber::unknown().into());
        assert_eq!(SequenceNumber::unknown(), SequenceNumber::from(-1i64 << 32));
    }

    #[test]
    fn add() {
        assert!(
            SequenceNumber::unknown()
                .checked_add(SequenceNumber::from(1))
                .is_none()
        );
        assert!(
            SequenceNumber::from(1)
                .checked_add(SequenceNumber::unknown())
                .is_none()
        );
        assert_eq!(
            Some(SequenceNumber::new(1, 1)),
            SequenceNumber::new(0, 0xffffffff).checked_add(SequenceNumber::new(0, 2))
        );

        assert_eq!(
            SequenceNumber::new(3, 7),
            SequenceNumber::new(1, 3) + SequenceNumber::new(2, 4)
        );
        assert_eq!(
            SequenceNumber::new(1, 10),
            SequenceNumber::new(0, 0xfffffff0) + SequenceNumber::new(0, 26)
        );
    }

    #[test]
    fn add_assign() {
        let mut n = SequenceNumber::zero();
        n += 1.into();
        n += 1.into();
        assert_eq!(SequenceNumber::new(0, 2), n);
    }

    #[test]
    #[should_panic]
    fn invalid_add() {
        SequenceNumber::unknown() + 1.into();
    }

    #[test]
    fn sub() {
        assert!(
            SequenceNumber::unknown()
                .checked_sub(SequenceNumber::from(1))
                .is_none()
        );
        assert!(
            SequenceNumber::from(1)
                .checked_sub(SequenceNumber::unknown())
                .is_none()
        );
        assert_eq!(
            Some(SequenceNumber::new(0, 0xffffffff)),
            SequenceNumber::new(1, 1).checked_sub(SequenceNumber::new(0, 2))
        );

        assert_eq!(
            SequenceNumber::new(1, 2),
            SequenceNumber::new(3, 5) - SequenceNumber::new(2, 3)
        );
        assert_eq!(
            SequenceNumber::new(1, 4),
            SequenceNumber::new(2, 3) - SequenceNumber::new(0, 0xffffffff)
        );
    }

    #[test]
    fn sub_assign() {
        let mut n = SequenceNumber::new(1, 5);
        n -= 1.into();
        n -= 1.into();
        assert_eq!(SequenceNumber::new(1, 3), n);
    }

    #[test]
    #[should_panic]
    fn invalid_sub() {
        SequenceNumber::unknown() - 1.into();
    }

    #[test]
    fn order() {
        assert!(SequenceNumber::new(0, 0) < SequenceNumber::new(0, 1));
        assert!(SequenceNumber::new(0, 1) < SequenceNumber::new(1, 0));
        assert!(SequenceNumber::new(1, 0) < SequenceNumber::new(1, 1));
    }
}
