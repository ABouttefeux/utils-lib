use std::{
    cmp::Ordering,
    ops::{Div, Mul, Sub},
};

use num_traits::{One, Unsigned, Zero};

#[must_use]
#[inline]
pub fn gcd<Number>(n1: Number, n2: Number) -> Number
where
    Number: Sub<Output = Number> + Ord + Zero + One + Unsigned + Clone,
{
    if n1 == Number::zero() || n2 == Number::zero() {
        Number::zero()
    } else if n1 == Number::one() || n2 == Number::one() {
        Number::one()
    } else {
        match n1.cmp(&n2) {
            Ordering::Equal => n1,
            Ordering::Greater => gcd(n1 - n2.clone(), n2),
            Ordering::Less => gcd(n1.clone(), n2 - n1),
        }
    }
}

#[must_use]
#[inline]
pub fn lcm<Number>(n1: Number, n2: Number) -> Number
where
    Number: Sub<Output = Number>
        + Ord
        + Zero
        + One
        + Unsigned
        + Mul<Output = Number>
        + Div<Output = Number>
        + Clone,
{
    if n1 == Number::zero() || n2 == Number::zero() {
        Number::zero()
    } else {
        n1.clone() * n2.clone() / gcd(n1, n2)
    }
}

/// Do the absolute difference of two numbers. In mathematical notation it is `|a-b|`.
///
/// # Example
/// ```
/// use utils_lib::abs_diff;
///
/// assert_eq!(abs_diff(2_u32, 6_u32), 4_u32);
/// assert_eq!(abs_diff(6_usize, 4_usize), 2_usize);
/// assert_eq!(abs_diff(16_u8, 6_u8), 10_u8);
/// assert_eq!(abs_diff(5_i32, 17_i32), 12_i32);
/// assert_eq!(abs_diff(9_i128, 9_i128), 0_i128);
/// assert_eq!(abs_diff(9_i128, -3_i128), 12_i128);
/// assert_eq!(abs_diff(-9_f64, 11_f64), 20_f64);
/// ```
#[must_use]
#[inline]
pub fn abs_diff<T>(n1: T, n2: T) -> T::Output
where
    T: PartialOrd + Sub<T>,
{
    if n1 > n2 {
        n1 - n2
    } else {
        n2 - n1
    }
}
