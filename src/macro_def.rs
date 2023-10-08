//! Contains the macro definitions

/// impl [`std::ops`] trait for a given type wrapper with a given method that
/// gives or is derefed into the wrapped value. The traits have to be imported by you at the place of invocation,
/// (see example).
///
/// # Example
/// ```
/// // this has to be imported for the macro to work.
/// use std::ops::{Add, AddAssign, Sub, SubAssign};
///
/// use utils_lib::impl_op_trait;
/// use utils_lib_derive::Getter;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Getter)]
/// struct Wrapper {
///     #[get(Const)]
///     #[get_mut(Pub)] // this create a method `float_mut` that gives `&mut f64`
///     float: f64,
/// }
///
/// impl std::ops::Deref for Wrapper {
///     type Target = f64;
///
///     fn deref(&self) -> &Self::Target {
///         self.float()
///     }
/// }
///
/// impl_op_trait!(Wrapper, float_mut, Add);
/// impl_op_trait!(Wrapper, float_mut, Sub);
///
/// let w1 = Wrapper { float: 1_f64 };
/// let w2 = Wrapper { float: 2_f64 };
///
/// let w3 = Wrapper { float: 3_f64 };
///
/// assert_eq!(w1 + w2, w3);
/// assert_eq!(w1 + &w2, w3);
/// assert_eq!(&w1 + w2, w3);
/// assert_eq!(&w1 + &w2, w3);
/// assert_eq!(w2 - w1, w1);
///
/// let mut w2 = w2;
/// w2 += w1;
///
/// assert_eq!(w2, w3);
/// ```
/// another possibility is that instead of using a direct mut getter we can use a
/// struct similar to [`crate::number::ValidationGuard`].
/// ```
/// // this has to be imported for the macro to work.
/// use std::ops::{Add, AddAssign, Sub, SubAssign};
/// use std::ops::{Deref, DerefMut};
///
/// use utils_lib::impl_op_trait;
/// use utils_lib_derive::Getter;
///
/// // We need Copy for the macro to work
/// #[derive(Debug, Clone, Copy, PartialEq, Getter)]
/// struct Wrapper {
///     #[get(Const)]
///     float: f64,
/// }
///
/// // we need this trait for the macro to work
/// impl Deref for Wrapper {
///     type Target = f64;
///
///     fn deref(&self) -> &Self::Target {
///         self.float()
///     }
/// }
///
/// #[derive(Debug, Getter)]
/// pub struct ValidationGuard<'a> {
///     reference: &'a mut Wrapper,
///     #[get(Const)]
///     #[get_mut]
///     float: f64,
/// }
///
/// impl<'a> Deref for ValidationGuard<'a> {
///     type Target = f64;
///
///     fn deref(&self) -> &Self::Target {
///         self.float()
///     }
/// }
///
/// // we need this trait for the macro to work
/// impl<'a> DerefMut for ValidationGuard<'a> {
///     fn deref_mut(&mut self) -> &mut Self::Target {
///         self.float_mut()
///     }
/// }
///
/// // when we drop the guard we change the value in the wrapper with the tracked value
/// // we could add some validation here
/// impl<'a> Drop for ValidationGuard<'a> {
///     fn drop(&mut self) {
///         self.reference.float = self.float;
///     }
/// }
///
/// impl Wrapper {
///     pub fn float_mut(&'_ mut self) -> ValidationGuard<'_> {
///         ValidationGuard {
///             float: self.float,
///             reference: self,
///         }
///     }
/// }
///
/// impl_op_trait!(Wrapper, float_mut, Add);
/// impl_op_trait!(Wrapper, float_mut, Sub);
///
/// let w1 = Wrapper { float: 1_f64 };
/// let w2 = Wrapper { float: 2_f64 };
///
/// let w3 = Wrapper { float: 3_f64 };
///
/// assert_eq!(w1 + w2, w3);
/// assert_eq!(w1 + &w2, w3);
/// assert_eq!(&w1 + w2, w3);
/// assert_eq!(&w1 + &w2, w3);
/// assert_eq!(w2 - w1, w1);
///
/// let mut w2 = w2;
/// w2 += &w1;
/// assert_eq!(w2, w3);
/// ```
// TODO resolve import issue
#[macro_export]
macro_rules! impl_op_trait {
    ($s:ty, $method:ident, Add) => {
        $crate::impl_op_trait!($s, $method, AddAssign, add_assign, Add, add);
    };
    ($s:ty, $method:ident, Mul) => {
        $crate::impl_op_trait!($s, $method, MulAssign, mul_assign, Mul, mul);
    };
    ($s:ty, $method:ident, Div) => {
        $crate::impl_op_trait!($s, $method, DivAssign, div_assign, Div, div);
    };
    ($s:ty, $method:ident, Sub) => {
        $crate::impl_op_trait!($s, $method, SubAssign, sub_assign, Sub, sub);
    };
    ($s:ty, $method:ident, Rem) => {
        $crate::impl_op_trait!($s, $method, RemAssign, rem_assign, Rem, rem);
    };
    ($s:ty, $method:ident, $t1:ident, $f1:ident, $t2:ident, $f2:ident) => {
        impl $t1 for $s {
            #[inline]
            fn $f1(&mut self, rhs: Self) {
                // rhs is marked mut but does not actually mutate
                //
                self.$method().$f1(*rhs);
            }
        }

        impl<'a> $t1<&'a $s> for $s {
            #[inline]
            fn $f1(&mut self, rhs: &'a Self) {
                self.$f1(*rhs);
            }
        }

        impl $t2 for $s {
            type Output = Self;

            #[inline]
            fn $f2(mut self, rhs: Self) -> Self::Output {
                self.$f1(rhs);
                self
            }
        }

        impl<'a> $t2<&'a $s> for $s {
            type Output = Self;

            #[inline]
            fn $f2(self, rhs: &'a Self) -> Self::Output {
                self.$f2(*rhs)
            }
        }

        impl<'a> $t2<$s> for &'a $s {
            type Output = $s;

            #[inline]
            fn $f2(self, rhs: $s) -> Self::Output {
                (*self).$f2(rhs)
            }
        }

        impl<'a, 'b> $t2<&'a $s> for &'b $s {
            type Output = $s;

            #[inline]
            fn $f2(self, rhs: &'a $s) -> Self::Output {
                (*self).$f2(*rhs)
            }
        }
    };
}
