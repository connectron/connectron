use std::{self, convert::{TryFrom, TryInto},
          ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
          time::{SystemTime, UNIX_EPOCH}};

use core::{Error, Result};

/// Type used to hold a timestamp.
///
/// The time origin corresponds to the Unix prime epoch 0h, 1 January 1970.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Time {
    seconds: i32,
    fraction: u32,
}

impl Time {
    #[inline]
    pub fn new(seconds: i32, fraction: u32) -> Self {
        Self { seconds, fraction }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        if self.seconds < 0 {
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn infinite() -> Self {
        Self::new(0x7fffffff, 0xffffffff)
    }

    #[inline]
    pub fn invalid() -> Self {
        Self::new(-1, 0xffffffff)
    }

    pub fn now() -> Result<Self> {
        SystemTime::now().duration_since(UNIX_EPOCH)?.try_into()
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    #[inline]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        if !self.is_valid() || !rhs.is_valid() {
            return None;
        }

        let (fraction, overflowed) = self.fraction.overflowing_add(rhs.fraction);
        if let Some(seconds) = self.seconds
            .checked_add(rhs.seconds)
            .and_then(|s| s.checked_add(if overflowed { 1 } else { 0 }))
        {
            Some(Self::new(seconds, fraction))
        } else {
            None
        }
    }

    #[inline]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        if !self.is_valid() || !rhs.is_valid() {
            return None;
        }

        let (fraction, overflowed) = self.fraction.overflowing_sub(rhs.fraction);
        if let Some(seconds) = self.seconds
            .checked_sub(rhs.seconds)
            .and_then(|s| s.checked_sub(if overflowed { 1 } else { 0 }))
        {
            if seconds < 0 {
                None
            } else {
                Some(Self::new(seconds, fraction))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        if !self.is_valid() || rhs < 0 {
            return None;
        }

        let total_fraction = self.fraction as u64 * rhs as u64;
        let extra_seconds = total_fraction / (std::u32::MAX as u64 + 1);
        let fraction = (total_fraction % ((std::u32::MAX as u64) + 1)) as u32;
        if let Some(seconds) = extra_seconds.try_into().ok().and_then(|es| {
            self.seconds
                .checked_mul(rhs)
                .and_then(|s| s.checked_add(es))
        }) {
            Some(Self::new(seconds, fraction))
        } else {
            None
        }
    }

    #[inline]
    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        if !self.is_valid() || rhs <= 0 {
            return None;
        }

        let seconds = self.seconds / rhs;
        let carry = self.seconds as u64 - seconds as u64 * (rhs as u64);
        let extra_fraction = carry * ((std::u32::MAX as u64) + 1) / (rhs as u64);
        let fraction = self.fraction / rhs as u32 + (extra_fraction as u32);
        Some(Self::new(seconds, fraction))
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect("add invalid times or overflow when adding times")
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect("subtract invalid times or overflow when subtracting times")
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for Time {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        self.checked_mul(rhs)
            .expect("multiply invalid time or overflow when multiplying time by scalar")
    }
}

impl MulAssign<i32> for Time {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl Div<i32> for Time {
    type Output = Time;

    fn div(self, rhs: i32) -> Time {
        self.checked_div(rhs)
            .expect("divide invalid time or divide by zero when dividing time by scalar")
    }
}

impl DivAssign<i32> for Time {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl TryFrom<std::time::Duration> for Time {
    type Error = Error;

    fn try_from(v: std::time::Duration) -> Result<Self> {
        let seconds = v.as_secs().try_into()?;
        let fraction = (((v.subsec_nanos() as u64) << 32) / 1_000_000_000) as _;
        Ok(Self::new(seconds, fraction))
    }
}

impl TryInto<std::time::Duration> for Time {
    type Error = Error;

    fn try_into(self) -> Result<std::time::Duration> {
        Ok(std::time::Duration::new(
            self.seconds.try_into()?,
            (((self.fraction as u64) * 1_000_000_000) >> 32) as _,
        ))
    }
}

/// Type used to hold time-differences.
pub type Duration = Time;

#[cfg(test)]
mod tests {
    use super::Time;

    #[test]
    fn time() {
        assert_eq!(
            Time {
                seconds: 1,
                fraction: 3,
            },
            Time::new(1, 3)
        );
        assert_eq!(
            Time {
                seconds: 0x7fffffff,
                fraction: 0xffffffff,
            },
            Time::infinite()
        );
        assert_eq!(
            Time {
                seconds: 0,
                fraction: 0,
            },
            Time::zero()
        );
    }

    #[test]
    fn add() {
        assert!(Time::invalid().checked_add(Time::new(0, 1)).is_none());
        assert!(Time::new(0, 1).checked_add(Time::invalid()).is_none());
        assert!(Time::infinite().checked_add(Time::new(0, 1)).is_none());
        assert!(Time::now().unwrap().checked_add(Time::new(0, 1)).is_some());
        assert_eq!(
            Some(Time::new(2, 10)),
            Time::new(1, 0xffffffff - 9).checked_add(Time::new(0, 20))
        );

        assert_eq!(Time::new(6, 10), Time::new(1, 3) + Time::new(5, 7));
        assert_eq!(
            Time::new(6, 10),
            Time::new(1, 0xfffffff0) + Time::new(4, 26)
        );
    }

    #[test]
    fn add_assign() {
        let mut t = Time::zero();
        t += Time::new(0, 1);
        t += Time::new(0, 1);
        assert_eq!(Time::new(0, 2), t);
    }

    #[test]
    #[should_panic]
    fn invalid_add() {
        Time::infinite() + Time::new(0, 1);
    }

    #[test]
    fn sub() {
        assert!(Time::invalid().checked_sub(Time::new(0, 1)).is_none());
        assert!(Time::new(0, 1).checked_sub(Time::invalid()).is_none());
        assert!(Time::zero().checked_sub(Time::new(0, 1)).is_none());
        assert!(Time::now().unwrap().checked_sub(Time::new(0, 1)).is_some());
        assert_eq!(
            Some(Time::new(1, 0xffffffff - 9)),
            Time::new(2, 10).checked_sub(Time::new(0, 20))
        );

        assert_eq!(Time::new(1, 3), Time::new(3, 9) - Time::new(2, 6));
        assert_eq!(
            Time::new(1, 0xfffffff0),
            Time::new(3, 10) - Time::new(1, 26)
        );
    }

    #[test]
    fn sub_assign() {
        let mut t = Time::new(1, 7);
        t -= Time::new(0, 1);
        t -= Time::new(0, 1);
        assert_eq!(Time::new(1, 5), t);
    }

    #[test]
    #[should_panic]
    fn invalid_sub() {
        Time::zero() - Time::new(0, 1);
    }

    #[test]
    fn mul() {
        assert!(Time::invalid().checked_mul(10).is_none());
        assert!(Time::new(0, 1).checked_mul(-10).is_none());
        assert!(Time::infinite().checked_mul(2).is_none());
        assert_eq!(Some(Time::zero()), Time::now().unwrap().checked_mul(0));
        assert_eq!(
            Some(Time::new(0x1f, 0xfff00000)),
            Time::new(1, 0xffff0000).checked_mul(0x10)
        );

        assert_eq!(Time::new(6, 12), Time::new(3, 6) * 2);
        assert_eq!(
            Time::new(0x1ff, 0xff000000),
            Time::new(1, 0xffff0000) * 0x100
        );
    }

    #[test]
    fn mul_assign() {
        let mut t = Time::new(1, 7);
        t *= 2;
        t *= 2;
        assert_eq!(Time::new(4, 28), t);
    }

    #[test]
    #[should_panic]
    fn invalid_mul() {
        Time::new(0, 1) * -1;
    }

    #[test]
    fn div() {
        assert!(Time::invalid().checked_div(10).is_none());
        assert!(Time::infinite().checked_div(0).is_none());
        assert!(Time::infinite().checked_div(-10).is_none());
        assert_eq!(Time::zero(), Time::zero().checked_div(10).unwrap());
        assert_eq!(
            Some(Time::new(1, 0xffff000)),
            Time::new(0x10, 0xffff0000).checked_div(0x10)
        );

        assert_eq!(Time::new(3, 9), Time::new(6, 18) / 2);
        assert_eq!(
            Time::new(0x1, 0xffffff00),
            Time::new(0x1ff, 0xffff0000) / 0x100
        );
    }

    #[test]
    fn div_assign() {
        let mut t = Time::new(8, 12);
        t /= 2;
        t /= 2;
        assert_eq!(Time::new(2, 3), t);
    }

    #[test]
    #[should_panic]
    fn invalid_div() {
        Time::new(0, 1) / -1;
    }

    #[test]
    fn order() {
        assert!(Time::new(0, 1) < Time::new(0, 2));
        assert!(Time::new(0, 1) < Time::new(1, 0));
        assert!(Time::new(1, 0) < Time::new(1, 1));
    }
}
