//! Contains [`PositiveFloat`].
//!
//! The module exits in order to compartmentalize code.

use std::{
    cmp::Ordering,
    fmt::{self, Display, LowerExp, UpperExp},
    hash::{Hash, Hasher},
    num::FpCategory,
    ops::Deref,
};

use num_traits::{One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{compare_f64, Validation, ValidationGuard};

/// A float that is `>= 0` and is not [`f64::NAN`] or [`f64::INFINITY`].
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

impl UpperExp for PositiveFloat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:E}", self.float())
    }
}

impl LowerExp for PositiveFloat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:e}", self.float())
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

    /// Create a new Self from a [`f64`]. It returns [`Some`] only if the float is valid ([`Self::validate_data`]), i.e.
    /// it is >= 0 it is not [`f64::NAN`] and not [`f64::INFINITY`].
    ///
    /// # Example
    /// ```
    /// use utils_lib::PositiveFloat;
    ///
    /// assert!(PositiveFloat::new(0_f64).is_some());
    /// assert!(PositiveFloat::new(2.5_f64).is_some());
    /// assert!(PositiveFloat::new(6.7E10_f64).is_some());
    ///
    /// assert_eq!(PositiveFloat::new(f64::INFINITY), None);
    /// assert_eq!(PositiveFloat::new(-f64::INFINITY), None);
    /// assert_eq!(PositiveFloat::new(-f64::NAN), None);
    /// assert_eq!(PositiveFloat::new(-1_f64), None);
    /// assert_eq!(PositiveFloat::new(-100_f64), None);
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
    /// use utils_lib::PositiveFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn test() -> Result<(), NoneError> {
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(0_f64),
    ///     PositiveFloat::new(0_f64).ok_or(NoneError)?
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(2.5_f64),
    ///     PositiveFloat::new(2.5_f64).ok_or(NoneError)?
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(6.7E10_f64),
    ///     PositiveFloat::new(6.7E10_f64).ok_or(NoneError)?
    /// );
    ///
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(f64::INFINITY),
    ///     PositiveFloat::default()
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(-f64::INFINITY),
    ///     PositiveFloat::default()
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(-f64::NAN),
    ///     PositiveFloat::default()
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(-1_f64),
    ///     PositiveFloat::default()
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(-100_f64),
    ///     PositiveFloat::default()
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
    pub fn float_mut(&'_ mut self) -> ValidationGuard<'_, Self> {
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
    /// use utils_lib::PositiveFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn test() -> Result<(), NoneError> {
    /// let p1 = PositiveFloat::new(1_f64).ok_or(NoneError)?;
    /// let p2 = PositiveFloat::new(2_f64).ok_or(NoneError)?;
    ///
    /// assert_eq!(p1.checked_sub(p2), None);
    /// assert_eq!(
    ///     p2.checked_sub(p1),
    ///     Some(PositiveFloat::new(1_f64).ok_or(NoneError)?)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Self::new(self.float() - other.float())
    }

    /// Do the subtraction of two [`PositiveFloat`] saturating at 0.
    /// It works in the same spirit as [`usize::saturating_sub`]
    ///
    /// # Example
    /// ```
    /// use utils_lib::PositiveFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn test() -> Result<(), NoneError> {
    /// let p1 = PositiveFloat::new(1_f64).ok_or(NoneError)?;
    /// let p2 = PositiveFloat::new(2_f64).ok_or(NoneError)?;
    ///
    /// assert_eq!(
    ///     p1.saturating_sub(p2),
    ///     PositiveFloat::new(0_f64).ok_or(NoneError)?
    /// );
    /// assert_eq!(
    ///     p2.saturating_sub(p1),
    ///     PositiveFloat::new(1_f64).ok_or(NoneError)?
    /// );
    /// # Ok(())
    /// # }
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

#[cfg(test)]
mod test {
    use crate::{error::NoneError, PositiveFloat};

    #[test]
    fn positive_float_const() -> Result<(), NoneError> {
        assert_eq!(
            PositiveFloat::default(),
            PositiveFloat::new(0_f64).ok_or(NoneError)?
        );

        assert_eq!(
            PositiveFloat::ZERO,
            PositiveFloat::new(0_f64).ok_or(NoneError)?
        );

        assert_eq!(
            PositiveFloat::ONE,
            PositiveFloat::new(1_f64).ok_or(NoneError)?
        );

        Ok(())
    }

    #[allow(clippy::float_cmp)] // reason = "This is fine, the test is made such that comparing float is ok."
    #[test]
    fn positive_float() -> Result<(), NoneError> {
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

        let mut t = PositiveFloat::new(1_f64).ok_or(NoneError)?;
        assert_eq!(*t.float_mut(), 1_f64);
        *t.float_mut() = 2_f64;
        assert_eq!(t.float(), 2_f64);
        *t.float_mut() = f64::NAN;
        assert_eq!(t.float(), 0_f64);

        Ok(())
    }

    #[test]
    fn saturating_sub() -> Result<(), NoneError> {
        let p1 = PositiveFloat::new(1_f64).ok_or(NoneError)?;
        let p2 = PositiveFloat::new(2_f64).ok_or(NoneError)?;

        assert_eq!(
            p1.saturating_sub(p2),
            PositiveFloat::new(0_f64).ok_or(NoneError)?
        );
        assert_eq!(
            p2.saturating_sub(p1),
            PositiveFloat::new(1_f64).ok_or(NoneError)?
        );

        Ok(())
    }
}
