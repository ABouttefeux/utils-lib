//! Contains [`PositiveFloat`].
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
use crate::ZeroOneBoundedFloat;

// TODO see if it is possible to use a trait to merge code of PositiveFloat and ZeroOneBoundedFloats.

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

/// represent in which range a  [`f64`] can be respectively to the bounds of [`PositiveFloat`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
enum BoundRange {
    /// [`f64::INFINITY`]
    UpperBound,
    /// between 0 and [`f64::MAX`]
    #[default]
    InRange,
    /// Strictly below 0
    LowerRange,
    /// Not a number
    Nan,
}

impl PositiveFloat {
    /// Value 0
    pub const ZERO: Self = Self(0_f64);

    /// Value 1
    pub const ONE: Self = Self(1_f64);

    /// Maximum value
    pub const MAX: Self = Self(f64::MAX);

    /// determine under which bound the given float is
    fn float_range(float: f64) -> BoundRange {
        if Self::validate_data(float) {
            BoundRange::InRange
        } else if float.is_nan() {
            BoundRange::Nan
        } else if float == f64::INFINITY {
            BoundRange::UpperBound
        } else {
            BoundRange::LowerRange
        }
    }

    // /// Create a wrapped value skipping the validity check
    // ///
    // /// # Safety
    // /// make sure that the float is valid
    // //# [cfg(any(not(debug_assertions), doc, test))]
    // // #[cfg_attr(debug_assertions, allow(dead_code))] // it is used by new_partially_check
    // #[must_use]
    // #[inline]
    // const unsafe fn new_unchecked(float: f64) -> Self {
    //     Self(float)
    // }

    // /// see the other [`Self::new_partially_check`]
    // // It is the other one which is documented
    // #[allow(clippy::missing_const_for_fn)] // it has to keep the same signature, the other fn can't be const
    // #[cfg(not(debug_assertions))]
    // #[must_use]
    // #[inline]
    // unsafe fn new_partially_check(float: f64) -> Self {
    //     Self::new_unchecked(float)
    // }

    // /// Create a wrapped value doing the validity check only when [`debug_assertions`]
    // /// and panics if the value is not valid. Otherwise it wraps the value even if it is not valid
    // ///
    // /// # Panic
    // /// Panics if the value if not valid and [`debug_assertions`] is on
    // ///
    // /// # Safety
    // /// make sure that the float is valid
    // #[cfg(debug_assertions)]
    // #[must_use]
    // #[inline]
    // unsafe fn new_partially_check(float: f64) -> Self {
    //     Self::new(float).expect("invalid value")
    // }

    /// Create a new Self from a [`f64`]. It returns [`Some`] only if the float is valid ([`Self::validate_data`]), i.e.
    /// it is >= 0 it is not [`f64::NAN`] and not [`f64::INFINITY`].
    ///
    /// # Errors
    ///
    /// - If `float` is smaller than zero it returns [`ConversionError::TooLow`].
    /// - If `float` is [`f64::INFINITY`] it returns [`ConversionError::Infinity`].
    /// - If `float` is [`f64::NAN`] it returns [`ConversionError::Nan`].
    ///
    /// # Example
    /// ```
    /// use utils_lib::number::PositiveFloatConversionError;
    /// use utils_lib::PositiveFloat;
    ///
    /// # fn main() -> Result<(), PositiveFloatConversionError> {
    /// PositiveFloat::new(0_f64)?;
    /// PositiveFloat::new(2.5_f64)?;
    /// PositiveFloat::new(6.7E10_f64)?;
    ///
    /// assert_eq!(
    ///     PositiveFloat::new(f64::INFINITY),
    ///     Err(PositiveFloatConversionError::Infinity)
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new(-f64::INFINITY),
    ///     Err(PositiveFloatConversionError::TooLow)
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new(-f64::NAN),
    ///     Err(PositiveFloatConversionError::Nan)
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new(-1_f64),
    ///     Err(PositiveFloatConversionError::TooLow)
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new(-100_f64),
    ///     Err(PositiveFloatConversionError::TooLow)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(float: f64) -> Result<Self, ConversionError> {
        match Self::float_range(float) {
            BoundRange::InRange => Ok(Self(float)),
            BoundRange::LowerRange => Err(ConversionError::TooLow),
            BoundRange::Nan => Err(ConversionError::Nan),
            BoundRange::UpperBound => Err(ConversionError::Infinity),
        }
    }

    /// Create a new Self with the float as value if it is valid ( `>= 0` finite and not [`f64::NAN`])
    /// or return the default value (0) instead.
    ///
    /// # Example
    /// ```
    /// use utils_lib::PositiveFloat;
    /// # use utils_lib::number::positive_float::ConversionError;
    ///
    /// # fn main() -> Result<(), ConversionError> {
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(0_f64),
    ///     PositiveFloat::new(0_f64)?
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(2.5_f64),
    ///     PositiveFloat::new(2.5_f64)?
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(6.7E10_f64),
    ///     PositiveFloat::new(6.7E10_f64)?
    /// );
    ///
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(f64::INFINITY),
    ///     PositiveFloat::ZERO
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

    // Create a new Self with the float as value if it is valid ( `>= 0` finite and not [`f64::NAN`])
    /// or return 0 or to [`f64::MAX`] if the value is infinity instead.
    ///
    /// Note that contrary to [`Self::new_or_default`] when given infinity it gives bac [`Self::MAX`]
    /// ```
    /// use utils_lib::PositiveFloat;
    /// # use utils_lib::error::NoneError;
    ///
    /// # fn main() -> Result<(), NoneError> {
    /// assert_eq!(
    ///     PositiveFloat::new_or_default(f64::INFINITY),
    ///     PositiveFloat::ZERO
    /// );
    /// assert_eq!(
    ///     PositiveFloat::new_or_bounded(f64::INFINITY),
    ///     PositiveFloat::MAX
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn new_or_bounded(float: f64) -> Self {
        match Self::float_range(float) {
            BoundRange::InRange => Self(float),
            BoundRange::UpperBound => Self::MAX,
            BoundRange::LowerRange | BoundRange::Nan => Self::ZERO,
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
    /// It is set to 0 or to [`f64::MAX`] if the value is infinity. See [`ValidationGuard`].
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
    /// # Errors
    ///
    /// See [`Self::new`]
    ///
    /// # Example
    ///
    /// ```
    /// use utils_lib::PositiveFloat;
    /// # use utils_lib::number::PositiveFloatConversionError;
    ///
    /// # fn main() -> Result<(), PositiveFloatConversionError> {
    /// let p1 = PositiveFloat::new(1_f64)?;
    /// let p2 = PositiveFloat::new(2_f64)?;
    ///
    /// assert_eq!(
    ///     p1.checked_sub(p2),
    ///     Err(PositiveFloatConversionError::TooLow)
    /// );
    /// assert_eq!(p2.checked_sub(p1), Ok(PositiveFloat::new(1_f64)?));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn checked_sub(self, other: Self) -> Result<Self, ConversionError> {
        Self::new(self.float() - other.float())
    }

    /// Do the subtraction of two [`PositiveFloat`] saturating at 0.
    /// It works in the same spirit as [`usize::saturating_sub`]
    ///
    /// # Example
    /// ```
    /// use utils_lib::PositiveFloat;
    /// # use utils_lib::number::PositiveFloatConversionError;
    ///
    /// # fn main() -> Result<(), PositiveFloatConversionError> {
    /// let p1 = PositiveFloat::new(1_f64)?;
    /// let p2 = PositiveFloat::new(2_f64)?;
    ///
    /// assert_eq!(p1.saturating_sub(p2), PositiveFloat::new(0_f64)?);
    /// assert_eq!(p2.saturating_sub(p1), PositiveFloat::new(1_f64)?);
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

/// Error for the conversion form a [`f64`] to a [`PositiveFloat`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ConversionError {
    /// The float is < 0
    TooLow,
    /// The float is [`f64::NAN`]
    Nan,
    /// The float is too big, i.e. [`f64::INFINITY`]
    Infinity,
}

impl Display for ConversionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infinity => write!(f, "the float is infinity"),
            Self::Nan => write!(f, "the float is not a number"),
            Self::TooLow => write!(f, "the float is below zero"),
        }
    }
}

impl Error for ConversionError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Infinity | Self::Nan | Self::TooLow => None,
        }
    }
}

impl From<ZeroOneBoundedFloat> for PositiveFloat {
    #[cfg(debug_assertions)]
    #[inline]
    fn from(value: ZeroOneBoundedFloat) -> Self {
        Self::new(value.float()).expect("the value could not be converted as it is not valid")
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn from(value: ZeroOneBoundedFloat) -> Self {
        //unsafe { Self::new_unchecked(value.float()) }
        Self::new_or_bounded(value.float())
    }
}

impl TryFrom<f64> for PositiveFloat {
    type Error = ConversionError;

    #[inline]
    fn try_from(float: f64) -> Result<Self, Self::Error> {
        Self::new(float)
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
        self.0 = match Self::float_range(float) {
            BoundRange::InRange => float,
            BoundRange::UpperBound => f64::MAX,
            BoundRange::LowerRange | BoundRange::Nan => 0_f64,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ConversionError, PositiveFloat};

    #[test]
    fn positive_float_const() -> Result<(), ConversionError> {
        assert_eq!(PositiveFloat::default(), PositiveFloat::new(0_f64)?);

        assert_eq!(PositiveFloat::ZERO, PositiveFloat::new(0_f64)?);

        assert_eq!(PositiveFloat::ONE, PositiveFloat::new(1_f64)?);

        Ok(())
    }

    #[allow(clippy::float_cmp)] // reason = "This is fine, the test is made such that comparing float is ok."
    #[test]
    fn positive_float() -> Result<(), ConversionError> {
        assert_eq!(
            PositiveFloat::new(f64::INFINITY),
            Err(ConversionError::Infinity)
        );
        assert_eq!(
            PositiveFloat::new(-f64::INFINITY),
            Err(ConversionError::TooLow)
        );
        assert_eq!(PositiveFloat::new(-f64::NAN), Err(ConversionError::Nan));
        assert_eq!(PositiveFloat::new(-1_f64), Err(ConversionError::TooLow));
        assert_eq!(PositiveFloat::new(-100_f64), Err(ConversionError::TooLow));
        assert_eq!(PositiveFloat::new(-0_f64), Ok(PositiveFloat::default()));
        PositiveFloat::new(1000_f64)?;
        PositiveFloat::new(2e32_f64)?;
        PositiveFloat::new(2e-32_f64)?;
        PositiveFloat::new(f64::MIN_POSITIVE)?;
        assert_eq!(PositiveFloat::new(-2e-32_f64), Err(ConversionError::TooLow));

        assert_eq!(
            PositiveFloat::new_or_bounded(f64::INFINITY),
            PositiveFloat::new(f64::MAX)?
        );

        assert_eq!(PositiveFloat::new_or_bounded(-1_f64), PositiveFloat::ZERO,);
        assert_eq!(PositiveFloat::new_or_bounded(1_f64), PositiveFloat::ONE);

        let mut t = PositiveFloat::new(1_f64)?;
        assert_eq!(*t.float_mut(), 1_f64);
        *t.float_mut() = 2_f64;
        assert_eq!(t.float(), 2_f64);
        *t.float_mut() = f64::NAN;
        assert_eq!(t.float(), 0_f64);
        *t.float_mut() = f64::INFINITY;
        assert_eq!(t.float(), f64::MAX);

        Ok(())
    }

    #[test]
    fn saturating_sub() -> Result<(), ConversionError> {
        let p1 = PositiveFloat::new(1_f64)?;
        let p2 = PositiveFloat::new(2_f64)?;

        assert_eq!(p1.saturating_sub(p2), PositiveFloat::new(0_f64)?);
        assert_eq!(p2.saturating_sub(p1), PositiveFloat::new(1_f64)?);

        Ok(())
    }
}
