//! Implementation of some [`std::ops`] trait for [`PositiveFloat`].
//!
//! more precisely [`std::ops::Add`], [`std::ops::AddAssign`], [`std::ops::Div`],
//! [`std::ops::DivAssign`], [`std::ops::Mul`] and [`std::ops::MulAssign`].

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

use super::{PositiveFloat, ZeroOneBoundedFloat};

impl_op_trait!(PositiveFloat, float_mut, Add);
impl_op_trait!(PositiveFloat, float_mut, Mul);
impl_op_trait!(PositiveFloat, float_mut, Div);
impl_op_trait!(PositiveFloat, float_mut, Rem);

impl_op_trait!(ZeroOneBoundedFloat, float_mut, Mul);
impl_op_trait!(ZeroOneBoundedFloat, float_mut, Rem);

// TODO macro and ref trait

impl MulAssign<ZeroOneBoundedFloat> for PositiveFloat {
    #[cfg(debug_assertions)]
    #[inline]
    fn mul_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        //*self.float_mut() *= rhs.float();
        *self = Self::new(self.float() * rhs.float()).expect("multiplication error");
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn mul_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        *self = Self::new_or_bounded(self.float() * rhs.float());
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
    #[cfg(debug_assertions)]
    #[inline]
    fn div_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        //*self.float_mut() /= rhs.float();
        *self = Self::new(self.float() / rhs.float()).expect("division error");
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn div_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        //*self.float_mut() /= rhs.float();
        *self = Self::new_or_bounded(self.float() / rhs.float());
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

    #[cfg(debug_assertions)]
    #[inline]
    fn div(self, rhs: PositiveFloat) -> Self::Output {
        PositiveFloat::new(self.float() / rhs.float()).expect("division error")
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn div(self, rhs: PositiveFloat) -> Self::Output {
        PositiveFloat::new_or_bounded(self.float() / rhs.float())
    }
}

//----------------------

impl AddAssign<ZeroOneBoundedFloat> for PositiveFloat {
    #[cfg(debug_assertions)]
    #[inline]
    fn add_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        //*self.float_mut() += rhs.float();
        *self = Self::new(self.float() + rhs.float()).expect("addition error");
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn add_assign(&mut self, rhs: ZeroOneBoundedFloat) {
        *self = Self::new_or_bounded(self.float() + rhs.float());
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

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{PositiveFloat, ZeroOneBoundedFloat};

    #[test]
    fn hybrid_mul() -> Result<(), Box<dyn Error>> {
        let mut p = PositiveFloat::new(5_f64)?;
        p *= ZeroOneBoundedFloat::ONE;
        assert_eq!(p, PositiveFloat::new(5_f64)?);
        p *= ZeroOneBoundedFloat::new(0.5_f64)?;
        assert_eq!(p, PositiveFloat::new(2.5_f64)?);
        p *= ZeroOneBoundedFloat::new(0.1_f64)?;
        assert_eq!(p, PositiveFloat::new(0.25_f64)?);
        p *= ZeroOneBoundedFloat::ZERO;
        assert_eq!(p, PositiveFloat::ZERO);

        let mut p = PositiveFloat::ZERO;
        p *= ZeroOneBoundedFloat::new(0.7_f64)?;
        assert_eq!(p, PositiveFloat::ZERO);

        assert_eq!(
            PositiveFloat::new(4_f64)? * ZeroOneBoundedFloat::new(0.5_f64)?,
            PositiveFloat::new(2_f64)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.5_f64)? * PositiveFloat::new(4_f64)?,
            PositiveFloat::new(2_f64)?
        );

        assert_eq!(
            PositiveFloat::new(10_f64)? * ZeroOneBoundedFloat::new(0.7_f64)?,
            PositiveFloat::new(7_f64)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.7_f64)? * PositiveFloat::new(10_f64)?,
            PositiveFloat::new(7_f64)?
        );

        assert_eq!(
            PositiveFloat::new(12_f64)? * ZeroOneBoundedFloat::new(0.3_f64)?,
            PositiveFloat::new(3.6_f64 - f64::EPSILON)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.3_f64)? * PositiveFloat::new(12_f64)?,
            PositiveFloat::new(3.6_f64 - f64::EPSILON)?
        );

        Ok(())
    }

    #[test]
    fn hybrid_div() -> Result<(), Box<dyn Error>> {
        let mut p = PositiveFloat::new(5_f64)?;
        p /= ZeroOneBoundedFloat::new(0.1)?;
        assert_eq!(p, PositiveFloat::new(50_f64)?);

        let mut p = PositiveFloat::new(5_f64)?;
        p /= ZeroOneBoundedFloat::new(0.5)?;
        assert_eq!(p, PositiveFloat::new(10_f64)?);

        let mut p = PositiveFloat::new(7_f64)?;
        p /= ZeroOneBoundedFloat::new(0.7)?;
        assert_eq!(p, PositiveFloat::new(10_f64)?);

        let mut p = PositiveFloat::new(0.1_f64)?;
        p /= ZeroOneBoundedFloat::new(0.5)?;
        assert_eq!(p, PositiveFloat::new(0.2_f64)?);

        assert_eq!(
            PositiveFloat::new(0.3_f64)? / ZeroOneBoundedFloat::new(0.5_f64)?,
            PositiveFloat::new(0.6_f64)?
        );
        assert_eq!(
            PositiveFloat::new(36_f64)? / ZeroOneBoundedFloat::new(0.3_f64)?,
            PositiveFloat::new(120_f64)?
        );
        assert_eq!(
            PositiveFloat::new(456_f64)? / ZeroOneBoundedFloat::new(0.12_f64)?,
            PositiveFloat::new(3800_f64)?
        );

        assert_eq!(
            ZeroOneBoundedFloat::new(0.6_f64)? / PositiveFloat::new(2_f64)?,
            PositiveFloat::new(0.3_f64)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.4_f64)? / PositiveFloat::new(0.2_f64)?,
            PositiveFloat::new(2_f64)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.1_f64)? / PositiveFloat::new(10_f64)?,
            PositiveFloat::new(0.01_f64)?
        );

        Ok(())
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "division error")]
    fn hybrid_div_zero_first() {
        let mut p = PositiveFloat::ONE;
        p /= ZeroOneBoundedFloat::ZERO;
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "division error")]
    fn hybrid_div_zero_second() {
        let _: PositiveFloat = PositiveFloat::ONE / ZeroOneBoundedFloat::ZERO;
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "division error")]
    fn hybrid_div_zero_third() {
        let _: PositiveFloat = ZeroOneBoundedFloat::ONE / PositiveFloat::ZERO;
    }

    #[test]
    fn hybrid_add() -> Result<(), Box<dyn Error>> {
        let mut p = PositiveFloat::new(5_f64)?;
        p += ZeroOneBoundedFloat::new(0.2_f64)?;
        assert_eq!(p, PositiveFloat::new(5.2_f64)?);
        p += ZeroOneBoundedFloat::new(0.7_f64)?;
        assert_eq!(p, PositiveFloat::new(5.9_f64)?);
        p += ZeroOneBoundedFloat::new(0.3_f64)?;
        assert_eq!(p, PositiveFloat::new(6.2_f64)?);
        p += ZeroOneBoundedFloat::new(0.8_f64)?;
        assert_eq!(p, PositiveFloat::new(7_f64)?);

        assert_eq!(
            PositiveFloat::new(9.5_f64)? + ZeroOneBoundedFloat::new(0.4_f64)?,
            PositiveFloat::new(9.9)?
        );
        assert_eq!(
            PositiveFloat::new(4_f64)? + ZeroOneBoundedFloat::new(0.9_f64)?,
            PositiveFloat::new(4.9)?
        );

        assert_eq!(
            ZeroOneBoundedFloat::new(1_f64)? + PositiveFloat::new(0.9_f64)?,
            PositiveFloat::new(1.9)?
        );
        assert_eq!(
            ZeroOneBoundedFloat::new(0.4_f64)? + PositiveFloat::new(1.9_f64)?,
            PositiveFloat::new(2.3)?
        );

        Ok(())
    }
}
