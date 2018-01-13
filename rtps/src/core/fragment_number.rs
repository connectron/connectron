use std::convert::{TryFrom, TryInto};

use core::error::{Error, ErrorKind, Result};

/// Type used to hold fragment numbers.
pub type FragmentNumber = u32;

/// Type used to hold information about individual fragment numbers within a range.
pub trait FragmentNumberSet {
    fn base(&self) -> FragmentNumber;
    fn num_bits(&self) -> u32;
    fn bitmaps(&self) -> Vec<u32>;
}

macro_rules! fragment_number_set {
    ( $name:ident, $bitmap_size:expr ) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        pub struct $name {
            base: FragmentNumber,
            num_bits: u32,
            bitmaps: [u32; $bitmap_size],
        }

        impl FragmentNumberSet for $name {
            fn base(&self) -> FragmentNumber {
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

fragment_number_set!(FragmentNumberSet1, 1);
fragment_number_set!(FragmentNumberSet2, 2);
fragment_number_set!(FragmentNumberSet3, 3);
fragment_number_set!(FragmentNumberSet4, 4);
fragment_number_set!(FragmentNumberSet5, 5);
fragment_number_set!(FragmentNumberSet6, 6);
fragment_number_set!(FragmentNumberSet7, 7);
fragment_number_set!(FragmentNumberSet8, 8);

impl TryFrom<Vec<FragmentNumber>> for Box<FragmentNumberSet> {
    type Error = Error;

    fn try_from(v: Vec<FragmentNumber>) -> Result<Self> {
        if v.len() < 1 || v.len() > 256 {
            return Err(ErrorKind::InvalidLength(v.len()).into());
        }

        let mut v = v;
        v.sort_unstable();

        let range = v.last().unwrap() - v.first().unwrap();
        if range > 255 {
            return Err(ErrorKind::InvalidRange.into());
        } else if *v.first().unwrap() < 1 {
            return Err(ErrorKind::InvalidValue.into());
        }

        let (mut bitmaps, _, map) = v.iter().skip(1).fold(
            (vec![], *v.first().unwrap(), 1u32),
            |mut acc, &n| {
                let (base, bitmap) = if n - acc.1 > 31 {
                    acc.0.push(acc.2);
                    (acc.1 + 32, 0u32)
                } else {
                    (acc.1, acc.2)
                };
                let shift = n - base;
                (acc.0, base, bitmap | (1u32 << shift))
            },
        );
        bitmaps.push(map);

        match range / 32 + 1 {
            1 => {
                let maps: &[u32; 1] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet1 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            2 => {
                let maps: &[u32; 2] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet2 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            3 => {
                let maps: &[u32; 3] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet3 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            4 => {
                let maps: &[u32; 4] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet4 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            5 => {
                let maps: &[u32; 5] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet5 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            6 => {
                let maps: &[u32; 6] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet6 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            7 => {
                let maps: &[u32; 7] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet7 {
                    base: *v.first().unwrap(),
                    num_bits: (range + 1) as _,
                    bitmaps: *maps,
                }))
            }
            8 => {
                let maps: &[u32; 8] = bitmaps.as_slice().try_into().unwrap();
                Ok(Box::new(FragmentNumberSet8 {
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
    use super::FragmentNumberSet;

    #[test]
    fn fragment_number_set() {
        use std::convert::TryInto;

        let set: Box<FragmentNumberSet> = (1u32..100)
            .map(|i| i * 2)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert_eq!(2, set.base());
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
