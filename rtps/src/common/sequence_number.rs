use std::{convert::{TryFrom, TryInto}, default::Default, ops::{Add, AddAssign, Sub, SubAssign}};

use common::error::{Error, ErrorKind, Result};

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
    pub fn into_inner(self) -> i64 {
        self.into()
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

impl Add<SequenceNumber> for SequenceNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect("invalid values or overflow when adding sequence numbers")
    }
}

impl<'a> Add<&'a SequenceNumber> for SequenceNumber {
    type Output = <SequenceNumber as Add<SequenceNumber>>::Output;

    fn add(self, rhs: &'a SequenceNumber) -> Self::Output {
        self.add(*rhs)
    }
}

impl<'a> Add<SequenceNumber> for &'a SequenceNumber {
    type Output = <SequenceNumber as Add<SequenceNumber>>::Output;

    fn add(self, rhs: SequenceNumber) -> Self::Output {
        SequenceNumber::add(*self, rhs)
    }
}

impl<'a, 'b> Add<&'a SequenceNumber> for &'b SequenceNumber {
    type Output = <SequenceNumber as Add<SequenceNumber>>::Output;

    fn add(self, rhs: &'a SequenceNumber) -> Self::Output {
        SequenceNumber::add(*self, *rhs)
    }
}

impl AddAssign for SequenceNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<'a> AddAssign<&'a SequenceNumber> for SequenceNumber {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.add_assign(*rhs)
    }
}

impl Sub for SequenceNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect("invalid values or overflow when subtracting sequence numbers")
    }
}

impl<'a> Sub<&'a SequenceNumber> for SequenceNumber {
    type Output = <SequenceNumber as Sub<SequenceNumber>>::Output;

    fn sub(self, rhs: &'a SequenceNumber) -> Self::Output {
        self.sub(*rhs)
    }
}

impl<'a> Sub<SequenceNumber> for &'a SequenceNumber {
    type Output = <SequenceNumber as Sub<SequenceNumber>>::Output;

    fn sub(self, rhs: SequenceNumber) -> Self::Output {
        SequenceNumber::sub(*self, rhs)
    }
}

impl<'a, 'b> Sub<&'a SequenceNumber> for &'b SequenceNumber {
    type Output = <SequenceNumber as Sub<SequenceNumber>>::Output;

    fn sub(self, rhs: &'a SequenceNumber) -> Self::Output {
        SequenceNumber::sub(*self, *rhs)
    }
}

impl SubAssign for SequenceNumber {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<'a> SubAssign<&'a SequenceNumber> for SequenceNumber {
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.sub_assign(*rhs)
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
        (self.high as i64) * 0x100000000 + self.low as i64
    }
}

/// Type used to hold information about individual sequence numbers within a range.
pub trait SequenceNumberSet {
    fn base(&self) -> SequenceNumber;
    fn num_bits(&self) -> u32;
    fn bitmaps(&self) -> Vec<u32>;
}

macro_rules! sequence_number_set {
    ( $name:ident, $bitmap_size:expr ) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        pub struct $name {
            base: SequenceNumber,
            num_bits: u32,
            bitmaps: [u32; $bitmap_size],
        }

        impl SequenceNumberSet for $name {
            fn base(&self) -> SequenceNumber {
                self.base
            }

            fn num_bits(&self) -> u32 {
                self.num_bits
            }

            fn bitmaps(&self) -> Vec<u32> {
                self.bitmaps.as_ref().into()
            }
        }
    };
}

sequence_number_set!(SequenceNumberSet1, 1);
sequence_number_set!(SequenceNumberSet2, 2);
sequence_number_set!(SequenceNumberSet3, 3);
sequence_number_set!(SequenceNumberSet4, 4);
sequence_number_set!(SequenceNumberSet5, 5);
sequence_number_set!(SequenceNumberSet6, 6);
sequence_number_set!(SequenceNumberSet7, 7);
sequence_number_set!(SequenceNumberSet8, 8);

impl TryFrom<Vec<SequenceNumber>> for Box<SequenceNumberSet> {
    type Error = Error;

    fn try_from(v: Vec<SequenceNumber>) -> Result<Self> {
        if v.len() < 1 || v.len() > 256 {
            return Err(ErrorKind::InvalidLength(v.len()).into());
        }

        let mut v = v;
        v.sort_unstable();

        let range = (v.last().unwrap() - v.first().unwrap()).into_inner();
        if range > 255 {
            return Err(ErrorKind::InvalidRange.into());
        } else if v.first().unwrap().into_inner() < 1 {
            return Err(ErrorKind::InvalidValue.into());
        }

        let (mut bitmaps, _, map) = v.iter().skip(1).fold(
            (vec![], v.first().unwrap().into_inner(), 1u32),
            |mut acc, &n| {
                let (base, bitmap) = if n.into_inner() - acc.1 > 31 {
                    acc.0.push(acc.2);
                    (acc.1 + 32, 0u32)
                } else {
                    (acc.1, acc.2)
                };
                let shift = n.into_inner() - base;
                (acc.0, base, bitmap | (1u32 << shift))
            },
        );
        bitmaps.push(map);

        match range / 32 + 1 {
            1 => {
                let maps: &[u32; 1] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet1 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            2 => {
                let maps: &[u32; 2] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet2 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            3 => {
                let maps: &[u32; 3] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet3 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            4 => {
                let maps: &[u32; 4] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet4 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            5 => {
                let maps: &[u32; 5] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet5 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            6 => {
                let maps: &[u32; 6] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet6 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            7 => {
                let maps: &[u32; 7] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet7 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            8 => {
                let maps: &[u32; 8] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(SequenceNumberSet8 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SequenceNumber, SequenceNumberSet};

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
        n += SequenceNumber::from(1);
        n += &SequenceNumber::from(1);
        assert_eq!(SequenceNumber::new(0, 2), n);
    }

    #[test]
    #[should_panic]
    fn invalid_add() {
        SequenceNumber::unknown() + SequenceNumber::from(1);
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
        n -= SequenceNumber::from(1);
        n -= &SequenceNumber::from(1);
        assert_eq!(SequenceNumber::new(1, 3), n);
    }

    #[test]
    #[should_panic]
    fn invalid_sub() {
        SequenceNumber::unknown() - SequenceNumber::from(1);
    }

    #[test]
    fn order() {
        assert!(SequenceNumber::new(0, 0) < SequenceNumber::new(0, 1));
        assert!(SequenceNumber::new(0, 1) < SequenceNumber::new(1, 0));
        assert!(SequenceNumber::new(1, 0) < SequenceNumber::new(1, 1));
    }

    #[test]
    fn sequence_number_set() {
        use std::convert::TryInto;

        let set: Box<SequenceNumberSet> = (1i64..100)
            .map(|i| SequenceNumber::from(i * 2))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert_eq!(SequenceNumber::from(2), set.base());
        assert_eq!(197, set.num_bits());
        assert_eq!(0b1010101010101010101010101010101, set.bitmaps()[0]);
        assert_eq!(0b1010101010101010101010101010101, set.bitmaps()[1]);
        assert_eq!(0b1010101010101010101010101010101, set.bitmaps()[2]);
        assert_eq!(0b1010101010101010101010101010101, set.bitmaps()[3]);
        assert_eq!(0b1010101010101010101010101010101, set.bitmaps()[4]);
        assert_eq!(0b1010101010101010101010101010101, set.bitmaps()[5]);
        assert_eq!(0b10101, set.bitmaps()[6]);
    }
}
