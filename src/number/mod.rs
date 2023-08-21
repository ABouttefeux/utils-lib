mod num_op_traits;

use std::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher}, // cspell: ignore Hasher
    num::FpCategory,
    ops::{Deref, DerefMut},
};

use num_traits::{One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A float that is >= 0 and is not NaN or infinity
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositiveFloat(f64);

impl Eq for PositiveFloat {}

impl Ord for PositiveFloat {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        compare_f64(self.float(), other.float())
    }
}

impl PartialOrd for PositiveFloat {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for PositiveFloat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.float())
    }
}

impl Hash for PositiveFloat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.float().to_bits());
    }
}

impl Deref for PositiveFloat {
    type Target = f64;

    #[inline]
    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PositiveFloat {
    /// Value 0
    pub const ZERO: Self = Self(0_f64);
    /// Value 1
    pub const ONE: Self = Self(1_f64);

    #[inline]
    #[must_use]
    pub fn new(float: f64) -> Option<Self> {
        Self::validate_data(float).then_some(Self(float))
    }

    #[inline]
    #[must_use]
    pub fn new_or_default(float: f64) -> Self {
        Self::new(float).unwrap_or_default()
    }

    /// Get the underling float. It could also be accessed by using [`Deref`],
    /// note that [`DerefMut`] is not implemented.
    #[inline]
    #[must_use]
    pub const fn float(self) -> f64 {
        self.0
    }

    /// Returns a way to mut the underlying float. If the final value is not valid,
    /// It is set to 0. See [`NumberValidationGuard`].
    #[inline]
    #[must_use]
    pub fn float_mut(&'_ mut self) -> ValidationGuard<'_, Self> {
        ValidationGuard {
            float: self.0,
            positive_float: self,
        }
    }

    #[inline]
    #[must_use]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Self::new(self.float() - other.float())
    }

    /// Do the subtraction of two [`PositiveFloat`] saturating at 0.
    ///
    /// # Example
    /// TODO
    /// ```
    /// use utils_lib::PositiveFloat;
    ///
    /// let p1 = PositiveFloat::new(1_f64).unwrap();
    /// let p2 = PositiveFloat::new(2_f64).unwrap();
    ///
    /// assert_eq!(p1.saturating_sub(p2), PositiveFloat::new(0_f64).unwrap());
    /// assert_eq!(p2.saturating_sub(p1), PositiveFloat::new(1_f64).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, other: Self) -> Self {
        self.checked_sub(other).unwrap_or_default()
    }
}

impl AsRef<f64> for PositiveFloat {
    #[inline]
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Zero for PositiveFloat {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.float() == 0_f64
    }
}

impl One for PositiveFloat {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl Validation for PositiveFloat {
    #[inline]
    fn validate_data(t: f64) -> bool {
        matches!(
            t.classify(),
            FpCategory::Normal | FpCategory::Subnormal | FpCategory::Zero
        ) && t >= 0_f64
    }

    #[inline]
    fn set_float(&mut self, float: f64) {
        self.0 = if Self::validate_data(float) {
            float
        } else {
            0_f64
        };
    }
}

/// Trait for type that have some validation step for data
pub trait Validation {
    /// return true if the data is valid for this struct
    #[must_use]
    fn validate_data(t: f64) -> bool;

    /// to set a float if it is valid, or the default value if it is not
    fn set_float(&mut self, float: f64);
}

//-----------------------------------

/// A structure created by [`PositiveFloat::float_mut`], it can be [`DerefMut`] as an `&mut f64`.
/// It ensure data validation on [`Drop`]. If the data is not valid it is set to 0.
///
/// We voluntarily do not have a new function. The guard is build by the wrapper.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ValidationGuard<'a, T: Validation + ?Sized> {
    /// the mut ref in order to "lock" the PositiveFloat and mutated on [`Drop`].
    #[serde(skip)]
    positive_float: &'a mut T,
    /// The new value
    float: f64,
}

impl<'a, T: Validation + ?Sized> ValidationGuard<'a, T> {
    /// a mut getter on the float
    #[inline]
    #[must_use]
    fn float_mut(&mut self) -> &mut f64 {
        &mut self.float
    }

    /// a getter on the value
    #[inline]
    #[must_use]
    const fn float(&self) -> &f64 {
        &self.float
    }
}

impl<'a, T: Validation + ?Sized> Deref for ValidationGuard<'a, T> {
    type Target = f64;

    #[inline]
    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.float()
    }
}

impl<'a, T: Validation + ?Sized> DerefMut for ValidationGuard<'a, T> {
    #[inline]
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // It is fine to do this way self.positive_float.0 is not accessible at this time as self.positive_float
        // is borrowed mutably to create the guard. Therefore no other ref exist to the data. There exists the point where
        // the data could be potentially be accessed unsafely by a pointer. This has the advantage of never putting invalidated data.
        // Yet old data could remain in the PositiveFloat.
        //
        // A wrong way to do it would be to mutate self.positive_float.0 and then validate it on drop.
        // As the guard could just be forgotten and never validate the data. In our case the data is never change but it is always valid.
        self.float_mut()
    }
}

impl<'a, T: Validation + ?Sized> Drop for ValidationGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.positive_float.set_float(self.float);
    }
}

impl<'a, T: Validation + ?Sized> Display for ValidationGuard<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.float)?;
        if T::validate_data(self.float) {
            Ok(())
        } else {
            write!(f, " (not valid)")
        }
    }
}

fn compare_f64(first: f64, other: f64) -> Ordering {
    match (first.classify(), other.classify()) {
        (FpCategory::Infinite, FpCategory::Infinite) => {
            #[allow(clippy::float_cmp)]
            // reason = "they are both either f64::INFINITY or f64::NEG_INFINITY"
            if first == other {
                Ordering::Equal
            } else if first == f64::INFINITY {
                // meaning other is - infinity
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        (FpCategory::Infinite, _) => {
            if first == f64::INFINITY {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        (_, FpCategory::Infinite) => {
            if other == f64::INFINITY {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        (FpCategory::Nan, FpCategory::Nan) => Ordering::Equal,
        (FpCategory::Nan, _) => panic!("comparing NaN with {other}"),
        (_, FpCategory::Nan) => panic!("comparing {first} with NaN"),
        (_, _) => first.partial_cmp(&other).expect("always is some"),
    }
}

//-----------------------------------

/// A float that f is  0 <= f <= 1 and is not NaN.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ZeroOneBoundedFloat(f64);

impl Eq for ZeroOneBoundedFloat {}

impl Ord for ZeroOneBoundedFloat {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        compare_f64(self.float(), other.float())
    }
}

impl PartialOrd for ZeroOneBoundedFloat {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for ZeroOneBoundedFloat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.float())
    }
}

impl Hash for ZeroOneBoundedFloat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.float().to_bits());
    }
}

impl Deref for ZeroOneBoundedFloat {
    type Target = f64;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ZeroOneBoundedFloat {
    pub const ZERO: Self = Self(0_f64);
    pub const ONE: Self = Self(1_f64);

    #[inline]
    #[must_use]
    pub fn new(float: f64) -> Option<Self> {
        Self::validate_data(float).then_some(Self(float))
    }

    /// Get the underling float. It could also be accessed by using [`Deref`],
    /// note that [`DerefMut`] is not implemented.
    #[inline]
    #[must_use]
    pub const fn float(self) -> f64 {
        self.0
    }

    /// Returns a way to mut the underlying float. If the final value is not valid,
    /// It is set to 0. See [`NumberValidationGuard`].
    #[inline]
    #[must_use]
    pub fn float_mut(&mut self) -> ValidationGuard<'_, Self> {
        ValidationGuard {
            float: self.0,
            positive_float: self,
        }
    }

    #[inline]
    #[must_use]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Self::new(self.float() - other.float())
    }

    /// Do the subtraction of two [`ZeroOneBoundedFloat`] saturating at 0.
    ///
    /// # Example
    /// TODO
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    ///
    /// let p1 = ZeroOneBoundedFloat::new(0.3_f64).unwrap();
    /// let p2 = ZeroOneBoundedFloat::new(0.6_f64).unwrap();
    ///
    /// assert_eq!(
    ///     p1.saturating_sub(p2),
    ///     ZeroOneBoundedFloat::new(0_f64).unwrap()
    /// );
    /// assert_eq!(
    ///     p2.saturating_sub(p1),
    ///     ZeroOneBoundedFloat::new(0.3_f64).unwrap()
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, other: Self) -> Self {
        self.checked_sub(other).unwrap_or_default()
    }

    // TODO doc
    #[inline]
    #[must_use]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Self::new(self.float() + other.float())
    }

    /// Do the addition of two [`ZeroOneBoundedFloat`] saturating at 1.
    ///
    /// # Example
    /// TODO
    #[inline]
    #[must_use]
    pub fn saturating_add(self, other: Self) -> Self {
        self.checked_add(other).unwrap_or(Self::ONE)
    }
}

impl AsRef<f64> for ZeroOneBoundedFloat {
    #[inline]
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl One for ZeroOneBoundedFloat {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl Validation for ZeroOneBoundedFloat {
    #[inline]
    fn validate_data(t: f64) -> bool {
        matches!(
            t.classify(),
            FpCategory::Normal | FpCategory::Subnormal | FpCategory::Zero
        ) && (0_f64..=1_f64).contains(&t)
    }

    #[inline]
    fn set_float(&mut self, float: f64) {
        self.0 = if Self::validate_data(float) {
            float
        } else if float.classify() != FpCategory::Nan && float >= 1_f64 {
            1_f64
        } else {
            // float is <= 0 or NaN
            0_f64
        };
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)] // reason = "unwrap can be tolerated in tests"
    use super::*;

    #[test]
    fn positive_float_default() {
        assert_eq!(PositiveFloat::default(), PositiveFloat::new(0_f64).unwrap());

        assert_eq!(PositiveFloat::ZERO, PositiveFloat::new(0_f64).unwrap());

        assert_eq!(PositiveFloat::ONE, PositiveFloat::new(1_f64).unwrap());

        assert_eq!(
            ZeroOneBoundedFloat::default(),
            ZeroOneBoundedFloat::new(0_f64).unwrap()
        );

        assert_eq!(
            ZeroOneBoundedFloat::ZERO,
            ZeroOneBoundedFloat::new(0_f64).unwrap()
        );

        assert_eq!(
            ZeroOneBoundedFloat::ONE,
            ZeroOneBoundedFloat::new(1_f64).unwrap()
        );
    }

    #[allow(clippy::float_cmp)] // reason = "This is fine, the test is made such that comparing float is ok."
    #[test]
    fn positive_float() {
        assert_eq!(PositiveFloat::new(f64::INFINITY), None);
        assert_eq!(PositiveFloat::new(-f64::INFINITY), None);
        assert_eq!(PositiveFloat::new(-f64::NAN), None);
        assert_eq!(PositiveFloat::new(-1_f64), None);
        assert_eq!(PositiveFloat::new(-100_f64), None);
        assert_eq!(PositiveFloat::new(-0_f64), Some(PositiveFloat::default()));
        assert!(PositiveFloat::new(1000_f64).is_some());
        assert!(PositiveFloat::new(2e32_f64).is_some());
        assert!(PositiveFloat::new(2e-32_f64).is_some());
        assert!(PositiveFloat::new(f64::MIN_POSITIVE).is_some());
        assert!(PositiveFloat::new(-2e-32_f64).is_none());

        let mut t = PositiveFloat::new(1_f64).unwrap();
        assert_eq!(*t.float_mut(), 1_f64);
        *t.float_mut() = 2_f64;
        assert_eq!(t.float(), 2_f64);
        *t.float_mut() = f64::NAN;
        assert_eq!(t.float(), 0_f64);
    }

    #[allow(clippy::float_cmp)] // reason = "This is fine, the test is made such that comparing float is ok."
    #[test]
    fn zero_one_bounded_float() {
        assert_eq!(ZeroOneBoundedFloat::new(f64::INFINITY), None);
        assert_eq!(ZeroOneBoundedFloat::new(-f64::INFINITY), None);
        assert_eq!(ZeroOneBoundedFloat::new(-f64::NAN), None);
        assert_eq!(ZeroOneBoundedFloat::new(-1_f64), None);
        assert_eq!(ZeroOneBoundedFloat::new(-100_f64), None);
        assert_eq!(
            ZeroOneBoundedFloat::new(-0_f64),
            Some(ZeroOneBoundedFloat::default())
        );
        assert_eq!(ZeroOneBoundedFloat::new(100_f64), None);
        assert_eq!(ZeroOneBoundedFloat::new(1.1_f64), None);
        assert!(ZeroOneBoundedFloat::new(1_f64).is_some());
        assert!(ZeroOneBoundedFloat::new(0.99_f64).is_some());
        assert!(ZeroOneBoundedFloat::new(0.9_f64).is_some());
        assert!(ZeroOneBoundedFloat::new(0.5_f64).is_some());

        let mut t = ZeroOneBoundedFloat::new(1_f64).unwrap();
        assert_eq!(*t.float_mut(), 1_f64);
        *t.float_mut() = 2_f64;
        assert_eq!(t.float(), 1_f64);
        *t.float_mut() = f64::NAN;
        assert_eq!(t.float(), 0_f64);
        t.set_float(f64::INFINITY);
        assert_eq!(t.float(), 1_f64);
        t.set_float(f64::NEG_INFINITY);
        assert_eq!(t.float(), 0_f64);
        t.set_float(1E+9_f64);
        assert_eq!(t.float(), 1_f64);
    }

    #[test]
    fn saturating_sub() {
        let p1 = PositiveFloat::new(1_f64).unwrap();
        let p2 = PositiveFloat::new(2_f64).unwrap();

        assert_eq!(p1.saturating_sub(p2), PositiveFloat::new(0_f64).unwrap());
        assert_eq!(p2.saturating_sub(p1), PositiveFloat::new(1_f64).unwrap());
    }
}
