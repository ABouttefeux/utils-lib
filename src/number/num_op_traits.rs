//! Implementation of some [`std::ops`] trait for [`PositiveFloat`].
//!
//! more precisely [`std::ops::Add`], [`std::ops::AddAssign`], [`std::ops::Div`],
//! [`std::ops::DivAssign`], [`std::ops::Mul`] and [`std::ops::MulAssign`].

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use super::{PositiveFloat, ZeroOneBoundedFloat};

impl_op_trait!(PositiveFloat, float_mut, Add);
impl_op_trait!(PositiveFloat, float_mut, Mul);
impl_op_trait!(PositiveFloat, float_mut, Div);

impl_op_trait!(ZeroOneBoundedFloat, float_mut, MulAssign, mul_assign, *=, Mul, mul, *);

// TODO macro and ref trait

impl MulAssign<ZeroOneBoundedFloat> for PositiveFloat {
    #[inline]
    fn mul_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        *self.float_mut() *= rhs.float();
    }
}

impl Mul<ZeroOneBoundedFloat> for PositiveFloat {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: ZeroOneBoundedFloat) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<PositiveFloat> for ZeroOneBoundedFloat {
    type Output = PositiveFloat;

    #[inline]
    fn mul(self, rhs: PositiveFloat) -> Self::Output {
        rhs * self
    }
}

//----------------------

impl DivAssign<ZeroOneBoundedFloat> for PositiveFloat {
    #[inline]
    fn div_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        *self.float_mut() /= rhs.float();
    }
}

impl Div<ZeroOneBoundedFloat> for PositiveFloat {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: ZeroOneBoundedFloat) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Div<PositiveFloat> for ZeroOneBoundedFloat {
    type Output = PositiveFloat;

    #[inline]
    fn div(self, rhs: PositiveFloat) -> Self::Output {
        let f = self.float() / rhs.float();
        PositiveFloat::new(f).expect("division error")
    }
}

//----------------------

impl AddAssign<ZeroOneBoundedFloat> for PositiveFloat {
    #[inline]
    fn add_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        *self.float_mut() += rhs.float();
    }
}

impl Add<ZeroOneBoundedFloat> for PositiveFloat {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: ZeroOneBoundedFloat) -> Self::Output {
        self += rhs;
        self
    }
}

impl Add<PositiveFloat> for ZeroOneBoundedFloat {
    type Output = PositiveFloat;

    #[inline]
    fn add(self, rhs: PositiveFloat) -> Self::Output {
        rhs + self
    }
}
