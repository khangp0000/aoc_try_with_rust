use num::traits::NumOps;
use std::fmt::{Debug, Display};
use std::num::ParseIntError;
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use std::str::FromStr;

pub trait IntegerRef<T>: NumOps<T, T> + for<'a> NumOps<&'a T, T> {}

pub trait IntegerShiftRef<U, T>:
    for<'a> Shl<&'a U, Output = T> + for<'a> Shr<&'a U, Output = T>
{
}

macro_rules! integer_trait {
    ($($t:ty)*) => {
        pub trait Integer:
            'static
            + num::PrimInt
            + num::Integer
            + num::FromPrimitive
            + num::traits::NumAssignRef
            + num::traits::NumRef
            + num::traits::RefNum<Self>
            + Debug
            + Display
            + Default
            + Send
            + Sync
            + FromStr<Err = ParseIntError>
            $(
            + Shl<$t, Output = Self>
            + for<'a> Shl<&'a $t, Output = Self>
            + Shr<$t, Output = Self>
            + for<'a> Shr<&'a $t, Output = Self>
            + ShlAssign<$t>
            + for<'a> ShlAssign<&'a $t>
            + ShrAssign<$t>
            + for<'a> ShrAssign<&'a $t>
            )+
        {
        }
    };
}
macro_rules! integer_ref_shift_impl {
    ($t:ty, $($u:ty)*) => ($(
        impl<'a> IntegerShiftRef<$u, $t> for &'a $t {}
    )*);
}

macro_rules! integer_impl {
    ($($t:ty)*) => ($(
        impl Integer for $t {}
        impl<'a> IntegerRef<$t> for &'a $t {}
        integer_ref_shift_impl!{$t, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 isize i128}
    )*);
}

integer_trait! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 isize i128 }

integer_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 isize i128 }
