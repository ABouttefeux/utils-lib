//! mod to separate the implementation of [`num_traits`] traits for [`ZeroOneBoundedFloat`]

use num_traits::{
    AsPrimitive, Bounded, CheckedMul, Inv, NumCast, One, Pow, SaturatingMul, ToBytes, ToPrimitive,
};

use super::ZeroOneBoundedFloat;
use crate::PositiveFloat;

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

impl<T: Copy + 'static> AsPrimitive<T> for ZeroOneBoundedFloat
where
    f64: AsPrimitive<T>,
{
    #[inline]
    fn as_(self) -> T {
        self.float().as_()
    }
}

impl ToPrimitive for ZeroOneBoundedFloat {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.float().to_i64()
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.float().to_u64()
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.float().to_u128()
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.float().to_i128()
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some(self.float())
    }
}

impl NumCast for ZeroOneBoundedFloat {
    #[inline]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        Self::new(n.to_f64()?).ok()
    }
}

// impl Unsigned for PositiveFloat {}

// impl Num for PositiveFloat {}

// impl NumOps for PositiveFloat {}

impl Pow<Self> for ZeroOneBoundedFloat {
    type Output = Self;

    #[inline]
    fn pow(self, rhs: Self) -> Self::Output {
        self.pow(<PositiveFloat as From<Self>>::from(rhs))
    }
}

impl Pow<PositiveFloat> for ZeroOneBoundedFloat {
    type Output = Self;

    #[cfg(debug_assertions)]
    #[inline]
    fn pow(self, rhs: PositiveFloat) -> Self::Output {
        Self::new(self.float().pow(rhs.float())).expect("value not valid")
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn pow(self, rhs: PositiveFloat) -> Self::Output {
        Self::new_or_bounded(self.float().pow(rhs.float()))
    }
}

impl Pow<f64> for ZeroOneBoundedFloat {
    type Output = PositiveFloat;

    #[inline]
    fn pow(self, rhs: f64) -> Self::Output {
        <PositiveFloat as From<Self>>::from(self).pow(rhs)
    }
}

impl ToBytes for ZeroOneBoundedFloat {
    type Bytes = <f64 as ToBytes>::Bytes;

    #[inline]
    fn to_be_bytes(&self) -> Self::Bytes {
        self.float().to_be_bytes()
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Bytes {
        self.float().to_le_bytes()
    }
}

// impl CheckedAdd for ZeroOneBoundedFloat {
//     #[inline]
//     fn checked_add(&self, v: &Self) -> Option<Self> {
//         Self::new(self.float() + v.float())
//     }
// }

// impl CheckedSub for PositiveFloat {}

impl CheckedMul for ZeroOneBoundedFloat {
    #[inline]
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Self::new(self.float() * v.float()).ok()
    }
}

// impl CheckedDiv for ZeroOneBoundedFloat {
//     #[inline]
//     fn checked_div(&self, v: &Self) -> Option<Self> {
//         if v.float() == 0_f64 {
//             None
//         } else {
//             Self::new(self.float() / v.float())
//         }
//     }
// }

// impl SaturatingAdd for ZeroOneBoundedFloat {
//     #[inline]
//     fn saturating_add(&self, v: &Self) -> Self {
//         Self::new_or_bounded(self.float() + v.float())
//     }
// }

// impl SaturatingSub for PositiveFloat {}

impl SaturatingMul for ZeroOneBoundedFloat {
    #[inline]
    fn saturating_mul(&self, v: &Self) -> Self {
        Self::new_or_bounded(self.float() * v.float())
    }
}

// impl WrappingAdd for PositiveFloat {}

// impl WrappingSub for PositiveFloat {}

// impl WrappingMul for PositiveFloat {}

impl Inv for ZeroOneBoundedFloat {
    type Output = <PositiveFloat as Inv>::Output;

    #[inline]
    fn inv(self) -> Self::Output {
        <PositiveFloat as From<Self>>::from(self).inv()
    }
}

#[cfg(test)]
mod test {
    use num_traits::{Bounded, CheckedMul, Inv, One, SaturatingMul};

    use super::ZeroOneBoundedFloat;
    use crate::number::ZeroOneBoundedFloatConversionError;

    #[allow(clippy::float_cmp)]
    #[test]
    fn zero() {
        assert!(ZeroOneBoundedFloat::one().is_one());
        assert_eq!(ZeroOneBoundedFloat::one(), ZeroOneBoundedFloat::ONE);
        assert_eq!(ZeroOneBoundedFloat::one().float(), 1_f64);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "cannot invert zero")]
    fn inv_zero() {
        ZeroOneBoundedFloat::ZERO.inv();
    }

    #[test]
    fn math_op() -> Result<(), ZeroOneBoundedFloatConversionError> {
        assert_eq!(ZeroOneBoundedFloat::max_value(), ZeroOneBoundedFloat::ONE);
        assert_eq!(ZeroOneBoundedFloat::min_value(), ZeroOneBoundedFloat::ZERO);

        assert_eq!(
            ZeroOneBoundedFloat::new(0.3_f64)?.checked_mul(&ZeroOneBoundedFloat::new(0.4_f64)?),
            Some(ZeroOneBoundedFloat::new(0.12_f64)?)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.5_f64)?.checked_mul(&ZeroOneBoundedFloat::new(0.2_f64)?),
            Some(ZeroOneBoundedFloat::new(0.1_f64)?)
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(1_f64)?.checked_mul(&ZeroOneBoundedFloat::new(1_f64)?),
            Some(ZeroOneBoundedFloat::new(1_f64)?)
        );

        assert_eq!(
            ZeroOneBoundedFloat::new(0.1_f64)?.saturating_mul(&ZeroOneBoundedFloat::new(0.3_f64)?),
            ZeroOneBoundedFloat::new(0.03)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.05_f64)?.saturating_mul(&ZeroOneBoundedFloat::new(0.5_f64)?),
            ZeroOneBoundedFloat::new(0.025_f64)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.5_f64)?.saturating_mul(&ZeroOneBoundedFloat::new(0.3_f64)?),
            ZeroOneBoundedFloat::new(0.15_f64)?
        );

        Ok(())
    }
}
