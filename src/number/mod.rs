//! Contains number and math utilities.

mod function;
mod num_op_traits;
pub mod positive_float;
pub mod sign;
pub mod zero_one_bounded_float;

use std::{
    cmp::Ordering,
    fmt::{self, Display},
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

/// Do an ordering operation on two [`f64`].
/// It is used internally for [`Ord`] and [`PartialOrd`] implementation of
/// [`ZeroOneBoundedFloat`] and [`PositiveFloat`]
///
/// # Panic
/// It panics if only value is [`f64:NAN`] and the other one is not either
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
