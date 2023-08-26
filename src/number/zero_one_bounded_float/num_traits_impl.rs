//! mod to separate the implementation of [`num_traits`] traits for [`ZeroOneBoundedFloat`]

use num_traits::{
    AsPrimitive, Bounded, CheckedMul, NumCast, One, SaturatingMul, ToBytes, ToPrimitive,
};

use super::ZeroOneBoundedFloat;

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
