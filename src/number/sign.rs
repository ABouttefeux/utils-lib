//! Contains the definition of [`Sign`] and related notions.

use std::{
    cmp::Ordering,
    fmt::{self, Display},
    num::FpCategory,
    ops::{Mul, MulAssign, Neg},
};

use serde::{Deserialize, Serialize};

// TODO conversion

/// Represent a sign.
#[allow(clippy::exhaustive_enums)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Sign {
    /// Strictly negative number (non zero)
    Negative = -1,
    /// Zero (or very close to zero)
    #[default]
    Zero = 0,
    /// Strictly positive number ( non zero)
    Positive = 1,
}

impl Sign {
    /// return a f64 form the sign `(-1_f64, 0_f64, 1_f64)`.
    #[must_use]
    #[inline]
    pub const fn to_f64(self) -> f64 {
        match self {
            Self::Negative => -1_f64,
            Self::Positive => 1_f64,
            Self::Zero => 0_f64,
        }
    }

    /// Get the sign form a f64.
    ///
    /// If the value is very close to zero but not quite the sing will nonetheless be [`Sign::Zero`].
    /// If f is NaN the sing will be [`Sign::Zero`].
    ///
    /// It may become constant once [`f64::classify`] and [`f64::is_sign_positive`] become
    /// constant as well.
    #[warn(clippy::missing_const_for_fn)]
    #[must_use]
    #[inline]
    pub fn sign_f64(f: f64) -> Self {
        // TODO abs_diff_eq!(f, 0_f64)
        if let FpCategory::Zero | FpCategory::Subnormal | FpCategory::Nan = f.classify() {
            Self::Zero
        } else if f.is_sign_positive() {
            Self::Positive
        } else {
            Self::Negative
        }
    }

    /// Convert the sign to an i8.
    #[must_use]
    #[inline]
    pub const fn to_i8(self) -> i8 {
        match self {
            Self::Negative => -1_i8,
            Self::Positive => 1_i8,
            Self::Zero => 0_i8,
        }
    }

    /// Get the sign of the given [`i8`]
    #[allow(clippy::comparison_chain)] // Cannot use cmp in const function
    #[must_use]
    #[inline]
    pub const fn sign_i8(n: i8) -> Self {
        if n == 0 {
            Self::Zero
        } else if n > 0 {
            Self::Positive
        } else {
            Self::Negative
        }
    }

    /// Returns the sign of `a - b`, where `a` and `b` are usize
    #[allow(clippy::comparison_chain)]
    #[must_use]
    #[inline]
    pub const fn sign_from_diff(a: usize, b: usize) -> Self {
        if a == b {
            Self::Zero
        } else if a > b {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

impl Display for Sign {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive => write!(f, "positive"),
            Self::Zero => write!(f, "zero"),
            Self::Negative => write!(f, "negative"),
        }
    }
}

impl From<Sign> for f64 {
    #[inline]
    fn from(s: Sign) -> Self {
        s.to_f64()
    }
}

impl From<f64> for Sign {
    #[inline]
    fn from(f: f64) -> Self {
        Self::sign_f64(f)
    }
}

impl From<Sign> for i8 {
    #[inline]
    fn from(s: Sign) -> Self {
        s.to_i8()
    }
}

impl From<i8> for Sign {
    #[inline]
    fn from(i: i8) -> Self {
        Self::sign_i8(i)
    }
}

impl Neg for Sign {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Positive => Self::Negative,
            Self::Zero => Self::Zero,
            Self::Negative => Self::Positive,
        }
    }
}

impl Mul for Sign {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Negative, Self::Negative) | (Self::Positive, Self::Positive) => Self::Positive,
            (Self::Zero, _) | (_, Self::Zero) => Self::Zero,
            (Self::Positive, Self::Negative) | (Self::Negative, Self::Positive) => Self::Negative,
        }
    }
}

impl MulAssign<Self> for Sign {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl PartialOrd for Sign {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sign {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_i8().cmp(&other.to_i8())
    }
}

// cspell: ignore levi civita
/// Return the levi civita symbol of the given index
/// # Example
/// ```
/// use utils_lib::number::sign::{levi_civita, Sign};
///
/// assert_eq!(Sign::Positive, levi_civita(&[1, 2, 3]));
/// assert_eq!(Sign::Negative, levi_civita(&[2, 1, 3]));
/// assert_eq!(Sign::Zero, levi_civita(&[2, 2, 3]));
/// ```
#[must_use]
#[inline]
pub const fn levi_civita(index: &[usize]) -> Sign {
    let mut prod = 1_i8;
    let mut i = 0_usize;
    while i < index.len() {
        let mut j = 0_usize;
        while j < i {
            prod *= Sign::sign_from_diff(index[i], index[j]).to_i8();
            j += 1;
        }
        i += 1;
    }
    Sign::sign_i8(prod)
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::{levi_civita, Sign};

    #[test]
    fn sign_i8() {
        assert_eq!(Sign::sign_i8(0), Sign::Zero);
        assert_eq!(Sign::sign_i8(-1), Sign::Negative);
        assert_eq!(Sign::sign_i8(1), Sign::Positive);
        assert_eq!(0, Sign::Zero.to_i8());
        assert_eq!(-1, Sign::Negative.to_i8());
        assert_eq!(1, Sign::Positive.to_i8());
    }

    #[test]
    fn levi_civita_test() {
        assert_eq!(Sign::Positive, levi_civita(&[]));
        assert_eq!(Sign::Positive, levi_civita(&[1, 2]));
        assert_eq!(Sign::Positive, levi_civita(&[0, 1]));
        assert_eq!(Sign::Positive, levi_civita(&[1, 2, 3]));
        assert_eq!(Sign::Positive, levi_civita(&[0, 1, 2]));
        assert_eq!(Sign::Positive, levi_civita(&[3, 1, 2]));
        assert_eq!(Sign::Positive, levi_civita(&[2, 3, 1]));
        assert_eq!(Sign::Positive, levi_civita(&[3, 1, 2, 4]));
        assert_eq!(Sign::Positive, levi_civita(&[1, 3, 4, 2]));
        assert_eq!(Sign::Zero, levi_civita(&[3, 3, 1]));
        assert_eq!(Sign::Zero, levi_civita(&[1, 1, 1]));
        assert_eq!(Sign::Zero, levi_civita(&[1, 1]));
        assert_eq!(Sign::Zero, levi_civita(&[2, 2]));
        assert_eq!(Sign::Negative, levi_civita(&[2, 1]));
        assert_eq!(Sign::Negative, levi_civita(&[1, 0]));
        assert_eq!(Sign::Negative, levi_civita(&[1, 3, 2]));
        assert_eq!(Sign::Negative, levi_civita(&[3, 2, 1]));
        assert_eq!(Sign::Negative, levi_civita(&[2, 1, 3]));
        assert_eq!(Sign::Negative, levi_civita(&[2, 1, 3, 4]));

        assert_eq!(Sign::Zero, Sign::sign_from_diff(0, 0));
        assert_eq!(Sign::Zero, Sign::sign_from_diff(4, 4));
        assert_eq!(Sign::Negative, Sign::sign_from_diff(1, 4));
        assert_eq!(Sign::Positive, Sign::sign_from_diff(4, 1));
    }

    #[allow(clippy::float_cmp)]
    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn sign() {
        assert_eq!(Sign::sign_f64(0_f64).to_f64(), 0_f64);
        assert_eq!(Sign::sign_f64(1_f64).to_f64(), 1_f64);
        assert_eq!(Sign::sign_f64(-1_f64).to_f64(), -1_f64);
        assert_eq!(Sign::sign_f64(34_f64), Sign::Positive);
        assert_eq!(Sign::sign_f64(-34_f64), Sign::Negative);
        assert_eq!(Sign::from(-34_f64), Sign::Negative);
        assert_eq!(f64::from(Sign::sign_f64(-1_f64)), -1_f64);
        assert_eq!(-Sign::Negative, Sign::Positive);
        assert_eq!(-Sign::Positive, Sign::Negative);
        assert_eq!(-Sign::Zero, Sign::Zero);

        assert_eq!(i8::from(Sign::from(0_i8)), 0_i8);
        assert_eq!(i8::from(Sign::from(1_i8)), 1_i8);
        assert_eq!(i8::from(Sign::from(-3_i8)), -1_i8);

        assert_eq!(Sign::default(), Sign::Zero);

        // mul
        assert_eq!(Sign::Positive * Sign::Positive, Sign::Positive);
        assert_eq!(Sign::Negative * Sign::Positive, Sign::Negative);
        assert_eq!(Sign::Positive * Sign::Negative, Sign::Negative);
        assert_eq!(Sign::Negative * Sign::Negative, Sign::Positive);

        assert_eq!(Sign::Zero * Sign::Positive, Sign::Zero);
        assert_eq!(Sign::Zero * Sign::Negative, Sign::Zero);
        assert_eq!(Sign::Positive * Sign::Zero, Sign::Zero);
        assert_eq!(Sign::Negative * Sign::Zero, Sign::Zero);

        let mut sign = Sign::Negative;
        sign *= Sign::Negative;
        assert_eq!(sign, Sign::Positive);

        // ord
        assert_eq!(Sign::Positive.cmp(&Sign::Zero), Ordering::Greater);
        assert_eq!(Sign::Positive.cmp(&Sign::Negative), Ordering::Greater);
        assert_eq!(Sign::Negative.cmp(&Sign::Zero), Ordering::Less);
        assert_eq!(Sign::Zero.cmp(&Sign::Zero), Ordering::Equal);

        assert_eq!(
            Sign::Positive.partial_cmp(&Sign::Zero),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Sign::Positive.partial_cmp(&Sign::Negative),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Sign::Negative.partial_cmp(&Sign::Zero),
            Some(Ordering::Less)
        );
        assert_eq!(Sign::Zero.partial_cmp(&Sign::Zero), Some(Ordering::Equal));

        // ---
        assert_eq!(Sign::Positive.to_string(), "positive");
        assert_eq!(Sign::Negative.to_string(), "negative");
        assert_eq!(Sign::Zero.to_string(), "zero");
    }
}
