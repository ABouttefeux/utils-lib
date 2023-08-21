/// impl Op trait for a given type
#[macro_export]
macro_rules! impl_op_trait {
    ($s:ident, $method:ident, Add) => {
        $crate::impl_op_trait!($s,$method, AddAssign, add_assign, +=, Add, add, +);
    };
    ($s:ident, $method:ident, Mul) => {
        $crate::impl_op_trait!($s,$method, MulAssign, mul_assign, *=, Mul, mul, *);
    };
    ($s:ident, $method:ident, Div) => {
        $crate::impl_op_trait!($s,$method, DivAssign, div_assign, /=, Div, div, /);
    };
    ($s:ident, $method:ident, Sub) => {
        $crate::impl_op_trait!($s,$method, SubAssign, sub_assign, -=, Sub, sub, -);
    };
    ($s:ident, $method:ident, $t1:ident, $f1:ident, $op1:tt, $t2:ident, $f2:ident, $op2:tt) => {
        impl $t1 for $s {
            #[inline]
            fn $f1(&mut self, rhs: Self) {
                *self.$method() $op1 *rhs;
            }
        }

        impl<'a> $t1<&'a $s> for $s {
            #[inline]
            fn $f1(&mut self, rhs: &'a Self) {
                *self $op1 *rhs;
            }
        }

        impl $t2 for $s {
            type Output = Self;

            #[inline]
            fn $f2(mut self, rhs: Self) -> Self::Output {
                self $op1 rhs;
                self
            }
        }

        impl<'a> $t2<&'a $s> for $s {
            type Output = Self;

            #[inline]
            fn $f2(self, rhs: &'a Self) -> Self::Output {
                self $op2 *rhs
            }
        }

        impl<'a> $t2<$s> for &'a $s {
            type Output = $s;

            #[inline]
            fn $f2(self, rhs: $s) -> Self::Output {
                *self $op2 rhs
            }
        }

        impl<'a, 'b> $t2<&'a $s> for &'b $s {
            type Output = $s;

            #[inline]
            fn $f2(self, rhs: &'a $s) -> Self::Output {
                *self $op2 *rhs
            }
        }
    };
}
