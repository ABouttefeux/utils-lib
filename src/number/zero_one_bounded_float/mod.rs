//! Contains [`ZeroOneBoundedFloat`].
//!
//! The module exits in order to compartmentalize code.

mod num_traits_impl;

use std::{
    cmp::Ordering,
    error::Error,
    fmt::{self, Display, LowerExp, UpperExp},
    hash::{Hash, Hasher},
    num::FpCategory,
    ops::Deref,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{compare_f64, Validation, ValidationGuard};
use crate::PositiveFloat;

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

/// represent in which range a  [`f64`] can be respectively to the bounds of [`ZeroOneBoundedFloat`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
enum BoundRange {
    /// Strictly above 1
    UpperBound,
    /// between 0 and 1
    #[default]
    InRange,
    /// Strictly below 0
    LowerRange,
    /// Not a number
    Nan,
}

impl ZeroOneBoundedFloat {
    /// Value 0
    pub const ZERO: Self = Self(0_f64);

    /// Value 1
    pub const ONE: Self = Self(1_f64);

    /// determine under which bound the given float is
    fn float_range(float: f64) -> BoundRange {
        if Self::validate_data(float) {
            BoundRange::InRange
        } else if float.is_nan() {
            BoundRange::Nan
        } else if float >= 1_f64 {
            BoundRange::UpperBound
        } else {
            BoundRange::LowerRange
        }
    }

    /// Create a wrapped value skipping the validity check
    ///
    /// # Safety
    /// make sure that the float is valid
    #[cfg_attr(debug_assertions, allow(dead_code))] // it is used by new_partially_check
    #[must_use]
    #[inline]
    const unsafe fn new_unchecked(float: f64) -> Self {
        Self(float)
    }

    /// Create a wrapped value doing the validity check only when [`debug_assertions`]
    /// and panics if the value is not valid. Otherwise it wraps the value even if it is not valid
    ///
    /// # Panic
    /// Panics if the value if not valid and [`debug_assertions`] is on
    ///
    /// # Safety
    /// make sure that the float is valid
    #[cfg(not(debug_assertions))]
    #[must_use]
    #[inline]
    unsafe fn new_partially_check(float: f64) -> Self {
        Self::new_unchecked(float)
    }

    /// Create a wrapped value doing the validity check only when [`debug_assertions`]
    /// and panics if the value is not valid. Otherwise it wraps the value even if it is not valid
    ///
    /// # Panic
    /// Panics if the value if not valid and [`debug_assertions`] is on
    ///
    /// # Safety
    /// make sure that the float is valid
    #[cfg(debug_assertions)]
    #[must_use]
    #[inline]
    unsafe fn new_partially_check(float: f64) -> Self {
        Self::new(float).expect("invalid value")
    }

    /// Create a new Self from a [`f64`]. It returns [`Some`] only if the float is valid ([`Self::validate_data`]), i.e.
    /// it is >= 0  and <= 1.
    ///
    /// # Example
    /// ```
    /// use utils_lib::{number::zero_one_bounded_float::ConversionError, ZeroOneBoundedFloat};
    ///
    /// assert!(ZeroOneBoundedFloat::new(0_f64).is_ok());
    /// assert!(ZeroOneBoundedFloat::new(0.001_f64).is_ok());
    /// assert!(ZeroOneBoundedFloat::new(0.6_f64).is_ok());
    /// assert!(ZeroOneBoundedFloat::new(1_f64).is_ok());
    ///
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new(2.5_f64),
    ///     Err(ConversionError::TooBig)
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new(6.7E10_f64),
    ///     Err(ConversionError::TooBig)
    /// );
    ///
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new(f64::INFINITY),
    ///     Err(ConversionError::TooBig)
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new(-f64::INFINITY),
    ///     Err(ConversionError::TooLow)
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new(-f64::NAN),
    ///     Err(ConversionError::Nan)
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new(-1_f64),
    ///     Err(ConversionError::TooLow)
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new(-100_f64),
    ///     Err(ConversionError::TooLow)
    /// );
    /// ```
    #[inline]
    pub fn new(float: f64) -> Result<Self, ConversionError> {
        match Self::float_range(float) {
            BoundRange::InRange => Ok(Self(float)),
            BoundRange::LowerRange => Err(ConversionError::TooLow),
            BoundRange::UpperBound => Err(ConversionError::TooBig),
            BoundRange::Nan => Err(ConversionError::Nan),
        }
    }

    /// Create a new Self with the float as value if it is valid ( `>= 0` and <= 1)
    /// or return the default value (0) instead.
    ///
    /// # Example
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::number::zero_one_bounded_float::ConversionError;
    ///
    /// # fn main() -> Result<(), ConversionError> {
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(0_f64),
    ///     ZeroOneBoundedFloat::new(0_f64)?
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(0.5_f64),
    ///     ZeroOneBoundedFloat::new(0.5_f64)?
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(1_f64),
    ///     ZeroOneBoundedFloat::new(1_f64)?
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

    // Create a new Self with the float as value if it is valid (`>= 0` and <= 1)
    /// or return 0 for value < 0 and 1 for value > 1
    ///
    /// Note that contrary to [`Self::new_or_default`] if values are > 1 it creates [`Self::ONE`]
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn main() -> Result<(), NoneError> {
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_default(1.5_f64),
    ///     ZeroOneBoundedFloat::ZERO
    /// );
    /// assert_eq!(
    ///     ZeroOneBoundedFloat::new_or_bounded(1.5_f64),
    ///     ZeroOneBoundedFloat::ONE
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn new_or_bounded(float: f64) -> Self {
        match Self::float_range(float) {
            BoundRange::InRange => Self(float),
            BoundRange::LowerRange | BoundRange::Nan => Self::ZERO,
            BoundRange::UpperBound => Self::ONE,
        }
    }

    /// Get the underling float. It could also be accessed by using [`Deref`],
    /// note that [`std::ops::DerefMut`] is not implemented.
    #[inline]
    #[must_use]
    pub const fn float(self) -> f64 {
        self.0
    }

    /// Returns a way to mut the underlying float. If the final value is not valid,
    /// It is set to 0. See [`ValidationGuard`].
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
    /// # Errors
    ///
    /// See [`Self::new`]
    ///
    /// # Example
    ///
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::number::ZeroOneBoundedFloatConversionError;
    ///
    /// # fn main() -> Result<(), ZeroOneBoundedFloatConversionError> {
    /// let p1 = ZeroOneBoundedFloat::new(0.3_f64)?;
    /// let p2 = ZeroOneBoundedFloat::new(0.6_f64)?;
    ///
    /// assert_eq!(
    ///     p1.checked_sub(p2),
    ///     Err(ZeroOneBoundedFloatConversionError::TooLow)
    /// );
    /// assert_eq!(p2.checked_sub(p1), Ok(ZeroOneBoundedFloat::new(0.3_f64)?));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn checked_sub(self, other: Self) -> Result<Self, ConversionError> {
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
    /// # Errors
    ///
    /// See [`Self::new`]
    ///
    /// # Example
    ///
    /// ```
    /// use utils_lib::number::ZeroOneBoundedFloatConversionError;
    /// use utils_lib::ZeroOneBoundedFloat;
    ///
    /// # fn main() -> Result<(), ZeroOneBoundedFloatConversionError> {
    /// let p1 = ZeroOneBoundedFloat::new(0.5_f64)?;
    /// let p2 = ZeroOneBoundedFloat::new(0.4_f64)?;
    /// let p3 = ZeroOneBoundedFloat::new(0.6_f64)?;
    ///
    /// assert_eq!(p1.checked_add(p2), Ok(ZeroOneBoundedFloat::new(0.9_f64)?));
    ///
    /// assert_eq!(
    ///     p1.checked_add(p3),
    ///     Err(ZeroOneBoundedFloatConversionError::TooBig)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn checked_add(self, other: Self) -> Result<Self, ConversionError> {
        Self::new(self.float() + other.float())
    }

    /// Do the addition of two [`ZeroOneBoundedFloat`] saturating at 1.
    /// It works in the same spirit as [`Self::saturating_sub`] but with the upper bound.
    ///
    /// # Example
    ///
    /// ```
    /// use utils_lib::ZeroOneBoundedFloat;
    /// # use utils_lib::number::zero_one_bounded_float::ConversionError;
    ///
    /// # fn main() -> Result<(), ConversionError> {
    /// let p1 = ZeroOneBoundedFloat::new(0.5_f64)?;
    /// let p2 = ZeroOneBoundedFloat::new(0.4_f64)?;
    /// let p3 = ZeroOneBoundedFloat::new(0.6_f64)?;
    ///
    /// assert_eq!(p1.saturating_add(p2), ZeroOneBoundedFloat::new(0.9_f64)?);
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

/// Error for the conversion form a [`f64`] to a [`ZeroOneBoundedFloat`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ConversionError {
    /// The float is < 0
    TooLow,
    /// The float is [`f64::NAN`]
    Nan,
    /// The float is too big, > 1
    TooBig,
}

impl Display for ConversionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooBig => write!(f, "the float is above one"),
            Self::Nan => write!(f, "the float is not a number"),
            Self::TooLow => write!(f, "the float is below zero"),
        }
    }
}

impl Error for ConversionError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::TooBig | Self::Nan | Self::TooLow => None,
        }
    }
}

impl TryFrom<PositiveFloat> for ZeroOneBoundedFloat {
    type Error = ConversionError;

    #[inline]
    fn try_from(value: PositiveFloat) -> Result<Self, Self::Error> {
        value.float().try_into()
    }
}

impl TryFrom<f64> for ZeroOneBoundedFloat {
    type Error = ConversionError;

    #[inline]
    fn try_from(float: f64) -> Result<Self, Self::Error> {
        Self::new(float)
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
        self.0 = match Self::float_range(float) {
            BoundRange::InRange => float,
            BoundRange::UpperBound => 1_f64,
            BoundRange::LowerRange | BoundRange::Nan => 0_f64,
        };
    }
}

#[cfg(test)]
mod test {
    use super::{super::Validation, ConversionError, ZeroOneBoundedFloat};
    use crate::error::NoneError;

    #[test]
    fn zero_one_bounded_float_const() -> Result<(), ConversionError> {
        assert_eq!(
            ZeroOneBoundedFloat::default(),
            ZeroOneBoundedFloat::new(0_f64)?
        );

        assert_eq!(ZeroOneBoundedFloat::ZERO, ZeroOneBoundedFloat::new(0_f64)?);

        assert_eq!(ZeroOneBoundedFloat::ONE, ZeroOneBoundedFloat::new(1_f64)?);

        Ok(())
    }

    #[allow(clippy::float_cmp)] // reason = "This is fine, the test is made such that comparing float is ok."
    #[test]
    fn zero_one_bounded_float() -> Result<(), ConversionError> {
        assert_eq!(
            ZeroOneBoundedFloat::new(f64::INFINITY),
            Err(ConversionError::TooBig)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(-f64::INFINITY),
            Err(ConversionError::TooLow)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(-f64::NAN),
            Err(ConversionError::Nan)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(-1_f64),
            Err(ConversionError::TooLow)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(-100_f64),
            Err(ConversionError::TooLow)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(-0_f64),
            Ok(ZeroOneBoundedFloat::default())
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(100_f64),
            Err(ConversionError::TooBig)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(1.1_f64),
            Err(ConversionError::TooBig)
        );
        ZeroOneBoundedFloat::new(1_f64)?;
        ZeroOneBoundedFloat::new(0.99_f64)?;
        ZeroOneBoundedFloat::new(0.9_f64)?;
        ZeroOneBoundedFloat::new(0.5_f64)?;

        assert_eq!(
            ZeroOneBoundedFloat::new_or_bounded(f64::INFINITY),
            ZeroOneBoundedFloat::ONE
        );

        assert_eq!(
            ZeroOneBoundedFloat::new_or_bounded(-1_f64),
            ZeroOneBoundedFloat::ZERO,
        );
        assert_eq!(
            ZeroOneBoundedFloat::new_or_bounded(1_f64),
            ZeroOneBoundedFloat::ONE
        );

        let mut t = ZeroOneBoundedFloat::new(1_f64)?;
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
    fn saturating_sub() -> Result<(), ConversionError> {
        let p1 = ZeroOneBoundedFloat::new(0.3_f64)?;
        let p2 = ZeroOneBoundedFloat::new(0.6_f64)?;

        assert_eq!(p1.saturating_sub(p2), ZeroOneBoundedFloat::new(0_f64)?);
        assert_eq!(p2.saturating_sub(p1), ZeroOneBoundedFloat::new(0.3_f64)?);

        Ok(())
    }
}
