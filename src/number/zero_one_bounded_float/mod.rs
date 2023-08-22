//! Contains [`ZeroOneBoundedFloat`].
//!
//! The module exits in order to compartmentalize code.

use std::{
    cmp::Ordering,
    fmt::{self, Display, LowerExp, UpperExp},
    hash::{Hash, Hasher},
    num::FpCategory,
    ops::Deref,
};

use num_traits::{Bounded, One};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{compare_f64, Validation, ValidationGuard};

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

impl UpperExp for ZeroOneBoundedFloat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:E}", self.float())
    }
}

impl LowerExp for ZeroOneBoundedFloat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:e}", self.float())
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
    /// Value 0
    pub const ZERO: Self = Self(0_f64);
    /// Value 1
    pub const ONE: Self = Self(1_f64);

    /// Create a new Self from a [`f64`]. It returns [`Some`] only if the float is valid ([`Self::validate_data`]), i.e.
    /// it is >= 0  and <= 1.
    ///
    /// # Example
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    ///
    /// assert!(ZeroOneBoundedFloat::new(0_f64).is_some());
    /// assert!(ZeroOneBoundedFloat::new(0.001_f64).is_some());
    /// assert!(ZeroOneBoundedFloat::new(0.6_f64).is_some());
    /// assert!(ZeroOneBoundedFloat::new(1_f64).is_some());
    ///
    /// assert_eq!(ZeroOneBoundedFloat::new(2.5_f64), None);
    /// assert_eq!(ZeroOneBoundedFloat::new(6.7E10_f64), None);
    ///
    /// assert_eq!(ZeroOneBoundedFloat::new(f64::INFINITY), None);
    /// assert_eq!(ZeroOneBoundedFloat::new(-f64::INFINITY), None);
    /// assert_eq!(ZeroOneBoundedFloat::new(-f64::NAN), None);
    /// assert_eq!(ZeroOneBoundedFloat::new(-1_f64), None);
    /// assert_eq!(ZeroOneBoundedFloat::new(-100_f64), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(float: f64) -> Option<Self> {
        Self::validate_data(float).then_some(Self(float))
    }

    /// Create a new Self with the float as value if it is valid ( `>= 0` finite and not [`f64::NAN`])
    /// or return the default value (0) instead.
    ///
    /// # Example
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn test() -> Result<(), NoneError> {
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(0_f64),
    ///     ZeroOneBoundedFloat::new(0_f64).ok_or(NoneError)?
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(0.5_f64),
    ///     ZeroOneBoundedFloat::new(0.5_f64).ok_or(NoneError)?
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(1_f64),
    ///     ZeroOneBoundedFloat::new(1_f64).ok_or(NoneError)?
    /// );
    ///
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(f64::INFINITY),
    ///     ZeroOneBoundedFloat::default()
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(-1_f64),
    ///     ZeroOneBoundedFloat::default()
    /// );
    /// # Ok(())
    /// # }
    /// ```
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
    pub fn float_mut(&mut self) -> ValidationGuard<'_, Self> {
        ValidationGuard {
            float: self.0,
            positive_float: self,
        }
    }

    /// Returns the value of the subtraction of two numbers if it doesn't underflow.
    /// It works in the same spirit as [`usize::checked_sub`].
    ///
    /// # Example
    ///
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn test() -> Result<(), NoneError> {
    /// let p1 = ZeroOneBoundedFloat::new(1_f64).ok_or(NoneError)?;
    /// let p2 = ZeroOneBoundedFloat::new(2_f64).ok_or(NoneError)?;
    ///
    /// assert_eq!(p1.checked_sub(p2), None);
    /// assert_eq!(
    ///     p2.checked_sub(p1),
    ///     Some(ZeroOneBoundedFloat::new(1_f64).ok_or(NoneError)?)
    /// );
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns the value of the addition of two numbers if it doesn't overflow.
    /// It works in the same spirit as [`Self::checked_sub`] but with the upper bound.
    ///
    /// # Example
    ///
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn test() -> Result<(), NoneError> {
    /// let p1 = ZeroOneBoundedFloat::new(0.5_f64).ok_or(NoneError)?;
    /// let p2 = ZeroOneBoundedFloat::new(0.4_f64).ok_or(NoneError)?;
    /// let p3 = ZeroOneBoundedFloat::new(0.6_f64).ok_or(NoneError)?;
    ///
    /// assert_eq!(
    ///     p1.checked_add(p2),
    ///     Some(ZeroOneBoundedFloat::new(0.9_f64).ok_or(NoneError)?)
    /// );
    ///
    /// assert_eq!(p1.checked_add(p3), None);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Self::new(self.float() + other.float())
    }

    /// Do the addition of two [`ZeroOneBoundedFloat`] saturating at 1.
    /// It works in the same spirit as [`Self::saturating_sub`] but with the upper bound.
    ///
    /// # Example
    ///
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn test() -> Result<(), NoneError> {
    /// let p1 = ZeroOneBoundedFloat::new(0.5_f64).ok_or(NoneError)?;
    /// let p2 = ZeroOneBoundedFloat::new(0.4_f64).ok_or(NoneError)?;
    /// let p3 = ZeroOneBoundedFloat::new(0.6_f64).ok_or(NoneError)?;
    ///
    /// assert_eq!(
    ///     p1.saturating_add(p2),
    ///     ZeroOneBoundedFloat::new(0.9_f64).ok_or(NoneError)?
    /// );
    ///
    /// assert_eq!(p1.saturating_add(p3), ZeroOneBoundedFloat::ONE);
    /// # Ok(())
    /// # }
    /// ```
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

impl Bounded for ZeroOneBoundedFloat {
    #[inline]
    fn min_value() -> Self {
        Self::ZERO
    }

    #[inline]
    fn max_value() -> Self {
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
    use super::{super::Validation, ZeroOneBoundedFloat};
    use crate::error::NoneError;

    #[test]
    fn zero_one_bounded_float_const() -> Result<(), NoneError> {
        assert_eq!(
            ZeroOneBoundedFloat::default(),
            ZeroOneBoundedFloat::new(0_f64).ok_or(NoneError)?
        );

        assert_eq!(
            ZeroOneBoundedFloat::ZERO,
            ZeroOneBoundedFloat::new(0_f64).ok_or(NoneError)?
        );

        assert_eq!(
            ZeroOneBoundedFloat::ONE,
            ZeroOneBoundedFloat::new(1_f64).ok_or(NoneError)?
        );

        Ok(())
    }

    #[allow(clippy::float_cmp)] // reason = "This is fine, the test is made such that comparing float is ok."
    #[test]
    fn zero_one_bounded_float() -> Result<(), NoneError> {
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

        let mut t = ZeroOneBoundedFloat::new(1_f64).ok_or(NoneError)?;
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

        Ok(())
    }

    #[test]
    fn saturating_sub() -> Result<(), NoneError> {
        let p1 = ZeroOneBoundedFloat::new(0.3_f64).ok_or(NoneError)?;
        let p2 = ZeroOneBoundedFloat::new(0.6_f64).ok_or(NoneError)?;

        assert_eq!(
            p1.saturating_sub(p2),
            ZeroOneBoundedFloat::new(0_f64).ok_or(NoneError)?
        );
        assert_eq!(
            p2.saturating_sub(p1),
            ZeroOneBoundedFloat::new(0.3_f64).ok_or(NoneError)?
        );

        Ok(())
    }
}
