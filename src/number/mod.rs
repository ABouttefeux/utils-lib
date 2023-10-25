//! Contains number and math utilities.

mod function;
mod num_op_traits;
pub mod positive_float;
pub mod sign;
pub mod zero_one_bounded_float;

use std::{
    cmp::Ordering,
    fmt::{self, Display, LowerExp, UpperExp},
    num::FpCategory,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "serde")]
use serde::Serialize;

// TODO conversion
// TODO num traits
pub use self::function::{abs_diff, gcd, lcm};
pub use self::positive_float::{ConversionError as PositiveFloatConversionError, PositiveFloat};
pub use self::sign::Sign;
pub use self::zero_one_bounded_float::{
    ConversionError as ZeroOneBoundedFloatConversionError, ZeroOneBoundedFloat,
};

/// Trait for type that have some validation step for data
pub trait Validation {
    /// return true if the data is valid for this struct
    #[must_use]
    fn validate_data(t: f64) -> bool;

    /// to set a float if it is valid, or the default value if it is not
    fn set_float(&mut self, float: f64);
}

//-----------------------------------

/// A structure created by [`PositiveFloat::float_mut`], it can be [`DerefMut`]
/// as an `&mut f64`.
/// It ensure data validation on [`Drop`]. If the data is not valid it is set to 0.
///
/// We voluntarily do not have a new function. The guard is build by the wrapper.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ValidationGuard<'a, T: Validation + ?Sized> {
    /// the mut ref in order to "lock" the PositiveFloat and mutated on [`Drop`].
    #[serde(skip)]
    reference: &'a mut T,
    /// The new value
    float: f64,
}

impl<'a, T> ValidationGuard<'a, T>
where
    T: Validation + ?Sized + AsRef<f64>,
{
    /// Create a new [`ValidationGuard`] from a mut reference.
    #[must_use]
    #[inline]
    pub fn new(reference: &'a mut T) -> Self {
        Self {
            float: *reference.as_ref(),
            reference,
        }
    }
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
        self.reference.set_float(self.float);
    }
}

impl<'a, T: Validation + ?Sized> Display for ValidationGuard<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <f64 as Display>::fmt(self.float(), f)?;
        if T::validate_data(self.float) {
            Ok(())
        } else {
            write!(f, " (not valid)")
        }
    }
}

impl<'a, T: Validation + ?Sized> UpperExp for ValidationGuard<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <f64 as UpperExp>::fmt(self.float(), f)?;
        if T::validate_data(self.float) {
            Ok(())
        } else {
            write!(f, " (not valid)")
        }
    }
}

impl<'a, T: Validation + ?Sized> LowerExp for ValidationGuard<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <f64 as LowerExp>::fmt(self.float(), f)?;
        if T::validate_data(self.float) {
            Ok(())
        } else {
            write!(f, " (not valid)")
        }
    }
}

impl<'a, T: Validation + ?Sized> AsRef<f64> for ValidationGuard<'a, T> {
    #[inline]
    fn as_ref(&self) -> &f64 {
        self.float()
    }
}

impl<'a, T: Validation + ?Sized> AsMut<f64> for ValidationGuard<'a, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut f64 {
        self.float_mut()
    }
}

impl<'a, T: Validation + ?Sized> From<ValidationGuard<'a, T>> for f64 {
    #[inline]
    fn from(value: ValidationGuard<'a, T>) -> Self {
        value.float
    }
}

impl<'a, T: Validation + ?Sized> From<&'a ValidationGuard<'a, T>> for &'a f64 {
    #[inline]
    fn from(value: &'a ValidationGuard<'a, T>) -> Self {
        value.float()
    }
}

impl<'a, 'b: 'a, T: Validation + ?Sized> From<&'a mut ValidationGuard<'b, T>> for &'a mut f64 {
    #[inline]
    fn from(value: &'a mut ValidationGuard<'b, T>) -> Self {
        value.float_mut()
    }
}

/// Do an ordering operation on two [`f64`].
/// It is used internally for [`Ord`] and [`PartialOrd`] implementation of
/// [`ZeroOneBoundedFloat`] and [`PositiveFloat`]
///
/// # Panic
/// It panics if only value is [`f64::NAN`] and the other one is not either
/// [`f64::INFINITY`] or [`f64::NEG_INFINITY`]
fn compare_f64(first: f64, other: f64) -> Ordering {
    match (first.classify(), other.classify()) {
        (FpCategory::Infinite, FpCategory::Infinite) => {
            #[allow(clippy::float_cmp)]
            // reason = "they are both either [`f64::INFINITY`] or [`f64::NEG_INFINITY`]"
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

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::{compare_f64, PositiveFloatConversionError};
    use crate::{PositiveFloat, ZeroOneBoundedFloat};

    #[test]
    fn cmp_f64() {
        // cmp number number
        assert_eq!(compare_f64(1.5_f64, 1.5_f64), Ordering::Equal);
        assert_eq!(compare_f64(0_f64, 0_f64), Ordering::Equal);
        assert_eq!(compare_f64(-5_f64, -5_f64), Ordering::Equal);

        assert_eq!(compare_f64(-5_f64, 1_f64), Ordering::Less);
        assert_eq!(compare_f64(1_f64, 2_f64), Ordering::Less);
        assert_eq!(compare_f64(-5_f64, -3_f64), Ordering::Less);

        assert_eq!(compare_f64(-5_f64, -30_f64), Ordering::Greater);
        assert_eq!(compare_f64(5_f64, 2_f64), Ordering::Greater);
        assert_eq!(compare_f64(50_f64, 1_f64), Ordering::Greater);

        //------
        // cmp inf inf

        assert_eq!(compare_f64(f64::INFINITY, f64::INFINITY), Ordering::Equal);
        assert_eq!(
            compare_f64(f64::INFINITY, f64::NEG_INFINITY),
            Ordering::Greater
        );
        assert_eq!(
            compare_f64(f64::NEG_INFINITY, f64::INFINITY),
            Ordering::Less
        );
        assert_eq!(
            compare_f64(f64::NEG_INFINITY, f64::NEG_INFINITY),
            Ordering::Equal
        );

        //------
        // cmp inf _

        assert_eq!(compare_f64(f64::INFINITY, f64::NAN), Ordering::Greater);
        assert_eq!(compare_f64(f64::INFINITY, 0_f64), Ordering::Greater);

        assert_eq!(compare_f64(f64::NEG_INFINITY, f64::NAN), Ordering::Less);
        assert_eq!(compare_f64(f64::NEG_INFINITY, 0_f64), Ordering::Less);

        //------
        // cmp _ inf

        assert_eq!(compare_f64(f64::NAN, f64::NEG_INFINITY), Ordering::Greater);
        assert_eq!(compare_f64(0_f64, f64::NEG_INFINITY), Ordering::Greater);

        assert_eq!(compare_f64(f64::NAN, f64::INFINITY), Ordering::Less);
        assert_eq!(compare_f64(0_f64, f64::INFINITY), Ordering::Less);

        //------
        // cmp Nan Nan

        assert_eq!(compare_f64(f64::NAN, f64::NAN), Ordering::Equal);
    }

    #[test]
    #[should_panic(expected = "comparing NaN with 0")]
    fn cmp_f64_fail_left() {
        compare_f64(f64::NAN, 0_f64);
    }

    #[test]
    #[should_panic(expected = "comparing 0 with NaN")]
    fn cmp_f64_fail_right() {
        compare_f64(0_f64, f64::NAN);
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn validation_guard_conversion() -> Result<(), PositiveFloatConversionError> {
        let mut p = PositiveFloat::ZERO;
        let mut guard = p.float_mut();

        assert_eq!(guard.as_ref(), &0_f64);
        assert_eq!(guard.as_mut(), &mut 0_f64);
        assert_eq!(Into::<&f64>::into(&guard), &0_f64);
        assert_eq!(Into::<&mut f64>::into(&mut guard), &mut 0_f64);
        let a = Into::<&mut f64>::into(&mut guard);
        *a = 1_f64;
        assert_eq!(Into::<f64>::into(guard), 1_f64);

        assert_eq!(p, PositiveFloat::ONE);
        assert_eq!(format!("{p}"), "1".to_owned());
        let mut guard = p.float_mut();
        assert_eq!(format!("{guard}"), "1".to_owned());
        *guard = -1_f64;
        assert_eq!(format!("{guard}"), "-1 (not valid)".to_owned());

        let mut z = ZeroOneBoundedFloat::ONE;
        let mut guard = z.float_mut();
        *guard = 2_f64;
        assert_eq!(format!("{guard}"), "2 (not valid)".to_owned());
        *guard = f64::NAN;
        assert_eq!(format!("{guard}"), "NaN (not valid)".to_owned());

        let mut p = PositiveFloat::new(0.123_456_f64)?;
        let mut guard = p.float_mut();

        assert_eq!(format!("{guard}"), "0.123456");
        assert_eq!(format!("{guard:e}"), "1.23456e-1");
        assert_eq!(format!("{guard:E}"), "1.23456E-1");
        assert_eq!(format!("{guard:.1}"), "0.1");
        assert_eq!(format!("{guard:.2}"), "0.12");

        *guard = -1.234_56E+10_f64;
        assert_eq!(format!("{guard}"), "-12345600000 (not valid)");
        assert_eq!(format!("{guard:e}"), "-1.23456e10 (not valid)");
        assert_eq!(format!("{guard:.1e}"), "-1.2e10 (not valid)");
        assert_eq!(format!("{guard:E}"), "-1.23456E10 (not valid)");
        assert_eq!(format!("{guard:.1E}"), "-1.2E10 (not valid)");

        Ok(())
    }
}
