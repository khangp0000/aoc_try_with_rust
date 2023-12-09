use derive_more::Display;
use num::PrimInt;
use std::cmp::max;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Range, RangeInclusive, Sub, SubAssign};
use thiserror::Error;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Display, Debug, Ord, PartialOrd)]
#[display("[{}..{}]", start, end)]
#[non_exhaustive]
pub struct IntRange<T: PrimInt> {
    pub start: T,
    pub end: T,
}

#[derive(Error, Debug)]
pub enum Error<T: PrimInt + Debug> {
    #[error("Invalid inclusive range from {0} to {1}")]
    InvalidRange(T, T),
}

impl<T: PrimInt + Debug> IntRange<T> {
    pub fn new(start: T, end: T) -> Result<Self, Error<T>> {
        if end < start {
            return Err(Error::InvalidRange(start, end));
        };
        return Ok(Self { start, end });
    }

    pub fn contains(&self, elem: &T) -> bool {
        return *elem >= self.start && *elem <= self.end;
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let (left, right) = if self.start < other.start {
            (self, other)
        } else {
            (other, self)
        };

        if left.end >= right.end {
            return Some(right.clone());
        }

        return Self::new(right.start, left.end).ok();
    }

    pub fn sub(&self, other: &Self) -> Vec<Self> {
        let mut res = Vec::new();
        if let Some(intersection) = self.intersect(other) {
            if intersection.start != self.start {
                res.push(Self::new(self.start, intersection.start - T::one()).unwrap());
            }
            if intersection.end != self.end {
                res.push(Self::new(intersection.end + T::one(), self.end).unwrap());
            }
        } else {
            res.push(self.clone());
        }

        return res;
    }

    pub fn coalesce(&self, other: &Self) -> Option<Self> {
        let (left, right) = if self.start < other.start {
            (self, other)
        } else {
            (other, self)
        };

        if left.end < right.start && (left.end + T::one() == right.start) {
            return Some(Self::new(left.start, right.end).ok().unwrap());
        } else if left.end >= right.start {
            return Some(Self::new(left.start, max(left.end, right.end)).unwrap());
        }

        return None;
    }
}

impl<'a, T: PrimInt> Add<T> for &'a IntRange<T> {
    type Output = IntRange<T>;

    fn add(self, rhs: T) -> Self::Output {
        return IntRange {
            start: self.start + rhs,
            end: self.end + rhs,
        };
    }
}

impl<'a, T: PrimInt> Sub<T> for &'a IntRange<T> {
    type Output = IntRange<T>;

    fn sub(self, rhs: T) -> Self::Output {
        return IntRange {
            start: self.start - rhs,
            end: self.end - rhs,
        };
    }
}

impl<T: PrimInt> Add<T> for IntRange<T> {
    type Output = IntRange<T>;

    fn add(self, rhs: T) -> Self::Output {
        return IntRange {
            start: self.start + rhs,
            end: self.end + rhs,
        };
    }
}

impl<T: PrimInt> Sub<T> for IntRange<T> {
    type Output = IntRange<T>;

    fn sub(self, rhs: T) -> Self::Output {
        return IntRange {
            start: self.start - rhs,
            end: self.end - rhs,
        };
    }
}

impl<'a, T: PrimInt> AddAssign<T> for &'a mut IntRange<T> {
    fn add_assign(&mut self, rhs: T) {
        self.start = self.start + rhs;
        self.end = self.end + rhs;
    }
}

impl<'a, T: PrimInt> SubAssign<T> for &'a mut IntRange<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.start = self.start - rhs;
        self.end = self.end - rhs;
    }
}

impl<T: PrimInt> AddAssign<T> for IntRange<T> {
    fn add_assign(&mut self, rhs: T) {
        self.start = self.start + rhs;
        self.end = self.end + rhs;
    }
}

impl<T: PrimInt> SubAssign<T> for IntRange<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.start = self.start - rhs;
        self.end = self.end - rhs;
    }
}

impl<T: PrimInt + Debug> From<&Range<T>> for IntRange<T> {
    fn from(value: &Range<T>) -> Self {
        return IntRange::new(value.start, value.end - T::one()).unwrap();
    }
}

impl<T: PrimInt + Debug> From<&RangeInclusive<T>> for IntRange<T> {
    fn from(value: &RangeInclusive<T>) -> Self {
        return IntRange::new(*value.start(), *value.end()).unwrap();
    }
}
