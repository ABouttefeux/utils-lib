//! mod to separate the implementation of [`num_traits`] traits for [`PositiveFloat`]

use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedMul, FloatConst, Inv, MulAdd,
    MulAddAssign, NumCast, One, Pow, SaturatingAdd, SaturatingMul, ToBytes, ToPrimitive, Zero,
};

use super::PositiveFloat;
use crate::ZeroOneBoundedFloat;

impl Zero for PositiveFloat {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.float().is_zero()
    }
}

impl One for PositiveFloat {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    // in next version this will be require
    #[inline]
    fn is_one(&self) -> bool {
        self.float().is_one()
    }
}

impl Bounded for PositiveFloat {
    #[inline]
    fn min_value() -> Self {
        Self::ZERO
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }
}

/// implement an item of the [`FloatConst`] trait for a wrapper
macro_rules! impl_float_const {
    ($fn:ident) => {
        #[cfg(debug_assertions)]
        #[inline]
        fn $fn() -> Self {
            Self::new(f64::$fn()).expect("always exist")
        }

        #[cfg(not(debug_assertions))]
        #[inline]
        fn $fn() -> Self {
            // SAFETY:
            // this is safe as the constant is in the bound
            // unsafe { Self::new_unchecked(f64::$fn()) }
            Self(f64::$fn())
        }
    };
}

#[allow(non_snake_case)] // require for the trait impl
impl FloatConst for PositiveFloat {
    impl_float_const!(E);
    impl_float_const!(FRAC_1_PI);
    impl_float_const!(FRAC_1_SQRT_2);
    impl_float_const!(FRAC_2_PI);
    impl_float_const!(FRAC_2_SQRT_PI);
    impl_float_const!(FRAC_PI_2);
    impl_float_const!(FRAC_PI_3);
    impl_float_const!(FRAC_PI_4);
    impl_float_const!(FRAC_PI_6);
    impl_float_const!(FRAC_PI_8);
    impl_float_const!(LN_10);
    impl_float_const!(LN_2);
    impl_float_const!(LOG10_E);
    impl_float_const!(LOG2_E);
    impl_float_const!(PI);
    impl_float_const!(SQRT_2);
}

impl<T: Copy + 'static> AsPrimitive<T> for PositiveFloat
where
    f64: AsPrimitive<T>,
{
    #[inline]
    fn as_(self) -> T {
        self.float().as_()
    }
}

impl ToPrimitive for PositiveFloat {
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

impl NumCast for PositiveFloat {
    #[inline]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        Self::new(n.to_f64()?).ok()
    }
}

// impl Unsigned for PositiveFloat {}

// impl Num for PositiveFloat {}

// impl NumOps for PositiveFloat {}

impl Pow<Self> for PositiveFloat {
    type Output = Self;

    #[inline]
    fn pow(self, rhs: Self) -> Self::Output {
        self.pow(rhs.float())
    }
}

impl Pow<ZeroOneBoundedFloat> for PositiveFloat {
    type Output = Self;

    #[inline]
    fn pow(self, rhs: ZeroOneBoundedFloat) -> Self::Output {
        self.pow(rhs.float())
    }
}

impl Pow<f64> for PositiveFloat {
    type Output = Self;

    #[cfg(debug_assertions)]
    #[inline]
    fn pow(self, rhs: f64) -> Self::Output {
        Self::new(self.float().pow(rhs)).expect("value not valid")
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn pow(self, rhs: f64) -> Self::Output {
        // unsafe { Self::new_unchecked(self.float().pow(rhs.float())) }
        Self::new_or_bounded(self.float().pow(rhs))
    }
}

impl ToBytes for PositiveFloat {
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

impl CheckedAdd for PositiveFloat {
    #[inline]
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Self::new(self.float() + v.float()).ok()
    }
}

// impl CheckedSub for PositiveFloat {}

impl CheckedMul for PositiveFloat {
    #[inline]
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Self::new(self.float() * v.float()).ok()
    }
}

impl CheckedDiv for PositiveFloat {
    #[inline]
    fn checked_div(&self, v: &Self) -> Option<Self> {
        if v.is_zero() {
            None
        } else {
            Self::new(self.float() / v.float()).ok()
        }
    }
}

impl Inv for PositiveFloat {
    type Output = Self;

    #[inline]
    fn inv(self) -> Self::Output {
        debug_assert!(!self.is_zero(), "cannot invert zero");
        Self::new_or_bounded(self.float().inv())
    }
}

impl MulAdd for PositiveFloat {
    type Output = Self;

    // TODO

    #[inline]
    #[cfg(debug_assertions)]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        let mul_add = self.float().mul_add(a.float(), b.float());
        Self::new(mul_add).expect("invalid value")
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        let mul_add = self.float().mul_add(a.float(), b.float());
        //unsafe { Self::new_unchecked(mul_add) }
        Self::new_or_bounded(mul_add)
    }
}

impl MulAddAssign for PositiveFloat {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

// impl OverflowingAdd for PositiveFloat {}

// impl OverflowingMul for PositiveFloat {}

// impl OverflowingSub for PositiveFloat {}

impl SaturatingAdd for PositiveFloat {
    #[inline]
    fn saturating_add(&self, v: &Self) -> Self {
        Self::new_or_bounded(self.float() + v.float())
    }
}

// impl SaturatingSub for PositiveFloat {}

impl SaturatingMul for PositiveFloat {
    #[inline]
    fn saturating_mul(&self, v: &Self) -> Self {
        Self::new_or_bounded(self.float() * v.float())
    }
}

// impl WrappingAdd for PositiveFloat {}

// impl WrappingSub for PositiveFloat {}

// impl WrappingMul for PositiveFloat {}

#[cfg(test)]
mod test {
    use std::error::Error;

    use num_traits::{
        Bounded, CheckedAdd, CheckedDiv, CheckedMul, FloatConst, Inv, One, Pow, SaturatingAdd,
        SaturatingMul, Zero,
    };

    use super::PositiveFloat;
    use crate::{number::PositiveFloatConversionError, ZeroOneBoundedFloat};

    #[allow(clippy::float_cmp)]
    #[test]
    fn num_const() {
        assert!(PositiveFloat::zero().is_zero());
        assert_eq!(PositiveFloat::zero(), PositiveFloat::ZERO);
        assert_eq!(PositiveFloat::zero().float(), 0_f64);

        assert!(PositiveFloat::one().is_one());
        assert_eq!(PositiveFloat::one(), PositiveFloat::ONE);
        assert_eq!(PositiveFloat::one().float(), 1_f64);
    }

    #[test]
    fn pow() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            PositiveFloat::ONE.pow(PositiveFloat::new(32_f64)?),
            PositiveFloat::ONE
        );

        assert_eq!(
            PositiveFloat::new(2_f64)?.pow(ZeroOneBoundedFloat::new(0.5_f64)?),
            PositiveFloat::SQRT_2()
        );
        assert_eq!(
            PositiveFloat::new(2_f64)?.pow(PositiveFloat::new(0.5_f64)?),
            PositiveFloat::SQRT_2()
        );

        assert_eq!(
            PositiveFloat::new(2_f64)?.pow(PositiveFloat::new(2_f64)?),
            PositiveFloat::new(4_f64)?
        );

        assert_eq!(
            ZeroOneBoundedFloat::new(0.5_f64)?.pow(PositiveFloat::new(2_f64)?),
            ZeroOneBoundedFloat::new(0.25_f64)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.5_f64)?.pow(2_f64),
            PositiveFloat::new(0.25_f64)?
        );

        assert!(
            (ZeroOneBoundedFloat::new(0.5_f64)?
                .pow(ZeroOneBoundedFloat::new(0.5_f64)?)
                .float()
                - ZeroOneBoundedFloat::new(0.5_f64.sqrt())?.float())
            .abs()
                < 1E-15_f64
        );

        Ok(())
    }

    #[test]
    fn inv() -> Result<(), PositiveFloatConversionError> {
        assert_eq!(PositiveFloat::ONE.inv(), PositiveFloat::ONE);
        assert_eq!(
            PositiveFloat::new(0.5_f64)?.inv(),
            PositiveFloat::new(2_f64)?
        );
        assert_eq!(
            PositiveFloat::new(2_f64)?.inv(),
            PositiveFloat::new(0.5_f64)?
        );
        assert_eq!(
            PositiveFloat::new(3_f64)?.inv(),
            PositiveFloat::new(1_f64 / 3_f64)?
        );
        assert_eq!(PositiveFloat::new(4_f64)?.inv(), PositiveFloat::new(0.25)?);
        assert_eq!(
            PositiveFloat::new(0.01_f64)?.inv(),
            PositiveFloat::new(100_f64)?
        );
        assert_eq!(
            PositiveFloat::new(0.001_f64)?.inv(),
            PositiveFloat::new(1000_f64)?
        );

        Ok(())
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "cannot invert zero")]
    fn inv_zero() {
        PositiveFloat::ZERO.inv();
    }

    #[test]
    fn math_op() -> Result<(), PositiveFloatConversionError> {
        assert_eq!(
            PositiveFloat::new(1_f64)? + PositiveFloat::new(4_f64)?,
            PositiveFloat::new(5_f64)?
        );

        assert_eq!(
            PositiveFloat::new(1_f64)?.checked_sub(PositiveFloat::new(4_f64)?),
            Err(PositiveFloatConversionError::TooLow)
        );
        assert_eq!(
            PositiveFloat::new(4_f64)?.checked_sub(PositiveFloat::new(1_f64)?),
            Ok(PositiveFloat::new(3_f64)?)
        );

        assert_eq!(
            PositiveFloat::new(1_f64)?.saturating_sub(PositiveFloat::new(4_f64)?),
            PositiveFloat::zero()
        );
        assert_eq!(
            PositiveFloat::new(4_f64)?.saturating_sub(PositiveFloat::new(1_f64)?),
            PositiveFloat::new(3_f64)?
        );

        assert_eq!(
            PositiveFloat::new(1_f64)?.saturating_add(&PositiveFloat::new(4_f64)?),
            PositiveFloat::new(5_f64)?
        );
        assert_eq!(
            PositiveFloat::new(4_f64)?.saturating_add(&PositiveFloat::new(1_f64)?),
            PositiveFloat::new(5_f64)?
        );
        assert_eq!(
            PositiveFloat::new(10_f64)?.saturating_add(&PositiveFloat::new(4_f64)?),
            PositiveFloat::new(14_f64)?
        );
        assert_eq!(
            PositiveFloat::new(f64::MAX)?.saturating_add(&PositiveFloat::new(f64::MAX)?),
            PositiveFloat::new(f64::MAX)?
        );

        assert_eq!(
            PositiveFloat::new(1_f64)?.checked_add(&PositiveFloat::new(4_f64)?),
            Some(PositiveFloat::new(5_f64)?)
        );
        assert_eq!(
            PositiveFloat::new(4_f64)?.checked_add(&PositiveFloat::new(1_f64)?),
            Some(PositiveFloat::new(5_f64)?)
        );
        assert_eq!(
            PositiveFloat::new(10_f64)?.checked_add(&PositiveFloat::new(4_f64)?),
            Some(PositiveFloat::new(14_f64)?)
        );
        assert_eq!(
            PositiveFloat::new(f64::MAX)?.checked_add(&PositiveFloat::new(f64::MAX)?),
            None
        );

        assert_eq!(
            PositiveFloat::new(3_f64)?.checked_mul(&PositiveFloat::new(4_f64)?),
            Some(PositiveFloat::new(12_f64)?)
        );
        assert_eq!(
            PositiveFloat::new(5_f64)?.checked_mul(&PositiveFloat::new(2_f64)?),
            Some(PositiveFloat::new(10_f64)?)
        );
        assert_eq!(
            PositiveFloat::new(5_f64)?.checked_mul(&PositiveFloat::new(f64::MAX)?),
            None
        );

        assert_eq!(
            PositiveFloat::new(3_f64)?.saturating_mul(&PositiveFloat::new(4_f64)?),
            PositiveFloat::new(12_f64)?
        );
        assert_eq!(
            PositiveFloat::new(5_f64)?.saturating_mul(&PositiveFloat::new(2_f64)?),
            PositiveFloat::new(10_f64)?
        );
        assert_eq!(
            PositiveFloat::new(5_f64)?.saturating_mul(&PositiveFloat::new(f64::MAX)?),
            PositiveFloat::new(f64::MAX)?
        );

        assert_eq!(
            PositiveFloat::new(3_f64)?.checked_div(&PositiveFloat::new(4_f64)?),
            Some(PositiveFloat::new(0.75)?)
        );
        assert_eq!(
            PositiveFloat::new(5_f64)?.checked_div(&PositiveFloat::new(2_f64)?),
            Some(PositiveFloat::new(2.5)?)
        );
        assert_eq!(
            PositiveFloat::new(5_f64)?.checked_div(&PositiveFloat::new(0_f64)?),
            None
        );

        assert_eq!(PositiveFloat::max_value(), PositiveFloat::MAX);
        assert_eq!(PositiveFloat::min_value(), PositiveFloat::ZERO);

        Ok(())
    }
}
