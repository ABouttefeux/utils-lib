//! mod to separate the implementation of [`num_traits`] traits for [`PositiveFloat`]

use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedMul, FloatConst, Inv, MulAdd,
    MulAddAssign, NumCast, One, Pow, SaturatingAdd, SaturatingMul, ToBytes, ToPrimitive, Zero,
};

use super::PositiveFloat;

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
            unsafe { Self::new_unchecked(f64::$fn()) }
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

    // TODO

    #[cfg(debug_assertions)]
    #[inline]
    fn pow(self, rhs: Self) -> Self::Output {
        Self::new(self.float().pow(rhs.float())).expect("value not valid")
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn pow(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.float().pow(rhs.float())) }
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
        if v.float() == 0_f64 {
            None
        } else {
            Self::new(self.float() / v.float()).ok()
        }
    }
}

impl Inv for PositiveFloat {
    type Output = Self;

    #[inline]
    fn inv(mut self) -> Self::Output {
        *self.float_mut() = self.float().inv();
        self
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
        unsafe { Self::new_unchecked(mul_add) }
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
