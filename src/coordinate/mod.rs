//! Module containing [`Coordinate`] a 2d coordinate and [`Axis2D`] an enumeration
//! of the x and y axis.

mod axis_2d;
mod iterator;

use std::{
    fmt::{
        self, Binary, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
    },
    iter::FusedIterator,
    ops::{Add, AddAssign, Index, IndexMut, Neg, Sub, SubAssign},
};

use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[doc(inline)]
pub use self::{axis_2d::Axis2D, iterator::CoordinateIterator};
use crate::number::abs_diff;

/// A two dimensional vector.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinate<T> {
    /// the x coordinate
    pub x: T,
    /// the y coordinate
    pub y: T,
}

impl<T> Coordinate<T> {
    /// Create a new [`Coordinate`] with two values for, respectively, the x and y coordinate.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Get the x coordinate.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> &T {
        &self.x
    }

    /// Get a mut reference on the x coordinate.
    #[inline]
    #[must_use]
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Get the y coordinate.
    #[inline]
    #[must_use]
    pub const fn y(&self) -> &T {
        &self.y
    }

    /// Get a mut reference on the y coordinate.
    #[inline]
    #[must_use]
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    /// Get the coordinate given by the [`Axis2D`] direction.
    #[inline]
    #[must_use]
    pub const fn get(&self, axis: Axis2D) -> &T {
        match axis {
            Axis2D::Vertical => self.x(),
            Axis2D::Horizontal => self.y(),
        }
    }

    /// Get a mutable reference on the coordinate given by the [`Axis2D`] direction.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, axis: Axis2D) -> &mut T {
        match axis {
            Axis2D::Vertical => self.x_mut(),
            Axis2D::Horizontal => self.y_mut(),
        }
    }

    // TODO own iterator for ExactSizeIterator
    /// Get an iterator on the coordinate elements
    #[inline]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &T> + DoubleEndedIterator + FusedIterator + ExactSizeIterator {
        self.into_iter()
    }

    /// Get an iterator on the coordinate elements as mutable reference
    #[inline]
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut T> + DoubleEndedIterator + FusedIterator + ExactSizeIterator
    {
        self.into_iter()
    }

    /// Get the [`Coordinate`] as a tuple references
    #[inline]
    #[must_use]
    pub const fn as_tuple(&self) -> (&T, &T) {
        (self.x(), self.y())
    }

    /// Get the [`Coordinate`] as a tuple mut references
    #[inline]
    #[must_use]
    pub fn as_tuple_mut(&mut self) -> (&mut T, &mut T) {
        (&mut self.x, &mut self.y)
    }

    /// Get the [`Coordinate`] as an array references
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> [&T; 2] {
        [self.x(), self.y()]
    }

    /// Get the [`Coordinate`] as an array mut references
    #[inline]
    #[must_use]
    pub fn as_array_mut(&mut self) -> [&mut T; 2] {
        [&mut self.x, &mut self.y]
    }

    /// Get the [`Coordinate`] as a [`Coordinate`] references
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> Coordinate<&T> {
        Coordinate::new(self.x(), self.y())
    }

    /// Get the [`Coordinate`] as a [`Coordinate`] mut references
    #[inline]
    #[must_use]
    pub fn as_mut(&mut self) -> Coordinate<&mut T> {
        Coordinate::new(&mut self.x, &mut self.y)
    }
}

/// Some "move" conversion function
impl<T> Coordinate<T> {
    /// Get the [`Coordinate`] as a tuple
    #[inline]
    #[must_use]
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    /// Get the [`Coordinate`] as an array
    #[inline]
    #[must_use]
    pub fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }
}

// ~const Drop
/// Const conversion function using [`Copy`] as a bound on `T`.
impl<T: Copy> Coordinate<T> {
    /// Get the [`Coordinate`] as a tuple.
    /// This is a const function.
    #[inline]
    #[must_use]
    pub const fn into_tuple_const(self) -> (T, T) {
        (self.x, self.y)
    }

    /// Get the [`Coordinate`] as an array.
    /// This is a const function.
    #[inline]
    #[must_use]
    pub const fn into_array_const(self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<'a, T> Coordinate<T>
where
    T: PartialOrd,
    &'a T: Sub + 'a,
    <&'a T as Sub>::Output: Add,
{
    /// Manhattan distances
    /// # Example
    ///
    /// ```
    /// use utils_lib::coordinate::Coordinate;
    ///
    /// let coord_zero = Coordinate::new(0_i32, 0_i32);
    /// assert_eq!(coord_zero.s1_distance(&coord_zero), 0_i32);
    ///
    /// let coord = Coordinate::new(0_i32, 1_i32);
    /// assert_eq!(coord.s1_distance(&coord_zero), 1_i32);
    ///
    /// let coord = Coordinate::new(1_i32, 0_i32);
    /// assert_eq!(coord.s1_distance(&coord_zero), 1_i32);
    ///
    /// let coord = Coordinate::new(3_i32, 4_i32);
    /// assert_eq!(coord.s1_distance(&coord_zero), 7_i32);
    ///
    /// let coord_1 = Coordinate::new(10_i32, 22_i32);
    /// let coord_2 = Coordinate::new(13_i32, 21_i32);
    /// assert_eq!(coord_1.s1_distance(&coord_2), 4_i32);
    /// assert_eq!(coord_2.s1_distance(&coord_1), 4_i32);
    /// ```
    #[inline]
    #[must_use]
    pub fn s1_distance(&'a self, other: &'a Self) -> <<&'a T as Sub>::Output as Add>::Output {
        abs_diff(self.x(), other.x()) + abs_diff(self.y(), other.y())
    }
}

//----------------------------------
// index operation

impl<T> Index<Axis2D> for Coordinate<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Axis2D) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<Axis2D> for Coordinate<T> {
    #[inline]
    fn index_mut(&mut self, index: Axis2D) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> Index<usize> for Coordinate<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array()[index]
    }
}

impl<T> IndexMut<usize> for Coordinate<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut()[index]
    }
}

// impl<T: Clone, I> Index<I> for Coordinate<T>
// where
//     [T; 2]: Index<I>,
// {
//     type Output = <[T; 2] as Index<I>>::Output;

//     #[inline]
//     fn index(&self, index: I) -> &Self::Output {
//         self.into_array().clone().index(index)
//     }
// }

//----------------------------------
// num operation

impl<T: AddAssign<T2>, T2> AddAssign<Coordinate<T2>> for Coordinate<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Coordinate<T2>) {
        *self.x_mut() += rhs.x;
        *self.y_mut() += rhs.y;
    }
}

impl<T: Add<T2>, T2> Add<Coordinate<T2>> for Coordinate<T> {
    type Output = Coordinate<T::Output>;

    #[inline]
    fn add(self, rhs: Coordinate<T2>) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: SubAssign<T2>, T2> SubAssign<Coordinate<T2>> for Coordinate<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Coordinate<T2>) {
        *self.x_mut() -= rhs.x;
        *self.y_mut() -= rhs.y;
    }
}

impl<T: Sub<T2>, T2> Sub<Coordinate<T2>> for Coordinate<T> {
    type Output = Coordinate<T::Output>;

    #[inline]
    fn sub(self, rhs: Coordinate<T2>) -> Self::Output {
        Coordinate::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Neg<Output = T2>, T2> Neg for Coordinate<T> {
    type Output = Coordinate<T2>;

    #[inline]
    fn neg(self) -> Self::Output {
        Coordinate::new(-self.x, -self.y)
    }
}

impl<T: Zero> Zero for Coordinate<T> {
    #[inline]
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.iter().all(Zero::is_zero)
    }
}

//----------------------------------
// conversion

impl<T> From<Coordinate<T>> for (T, T) {
    #[inline]
    fn from(value: Coordinate<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> From<(T, T)> for Coordinate<T> {
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<Coordinate<T>> for [T; 2] {
    #[inline]
    fn from(value: Coordinate<T>) -> Self {
        [value.x, value.y]
    }
}

#[allow(clippy::fallible_impl_from)] // reason = "the conversion actually never panic"
impl<T> From<[T; 2]> for Coordinate<T> {
    #[inline]
    fn from(value: [T; 2]) -> Self {
        let mut iter = value.into_iter();
        Self::new(
            iter.next().expect("never none"),
            iter.next().expect("never none"),
        )
    }
}

impl<T: Clone + Default> From<&[T]> for Coordinate<T> {
    #[inline]
    fn from(value: &[T]) -> Self {
        let mut iter = value.iter();
        Self::new(
            iter.next().cloned().unwrap_or_default(),
            iter.next().cloned().unwrap_or_default(),
        )
    }
}

impl<T: Default> From<Vec<T>> for Coordinate<T> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        let mut iter = value.into_iter();
        Self::new(
            iter.next().unwrap_or_default(),
            iter.next().unwrap_or_default(),
        )
    }
}

//----------------------------------
// format

/// implement a [`fmt`] trait for [`Coordinate`]
macro_rules! impl_fmt_coord {
    ($trait:path) => {
        impl<T: $trait> $trait for Coordinate<T> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "[")?;
                <T as $trait>::fmt(self.x(), f)?;
                write!(f, ", ")?;
                <T as $trait>::fmt(self.y(), f)?;
                write!(f, "]")
            }
        }
    };
}

impl_fmt_coord!(Display);
impl_fmt_coord!(Octal);
impl_fmt_coord!(LowerHex);
impl_fmt_coord!(UpperHex);
impl_fmt_coord!(Pointer);
impl_fmt_coord!(Binary);
impl_fmt_coord!(LowerExp);
impl_fmt_coord!(UpperExp);

#[cfg(test)]
mod test {

    use num_traits::Zero;

    use super::{Axis2D, Coordinate};
    use crate::{error::NoneError, PositiveFloat};

    #[test]
    fn axis_2d() {
        assert_eq!(!Axis2D::Vertical, Axis2D::Horizontal);
        assert_eq!(!Axis2D::Horizontal, Axis2D::Vertical);

        assert_eq!(Into::<usize>::into(Axis2D::Vertical), 0_usize);
        assert_eq!(Into::<usize>::into(Axis2D::Horizontal), 1_usize);

        assert_eq!(
            Into::<Coordinate<usize>>::into(Axis2D::Vertical),
            Coordinate::new(1_usize, 0_usize)
        );

        assert_eq!(Axis2D::Vertical.as_ref(), &0_usize);

        assert_eq!(Axis2D::try_from(2_usize), Err(NoneError));
        assert_eq!(Axis2D::try_from(1_usize), Ok(Axis2D::Horizontal));
    }

    #[test]
    fn coord() {
        let mut coord = Coordinate::new(0_usize, 1_usize);
        assert_eq!(coord.get(Axis2D::Vertical), &0_usize);
        assert_eq!(coord.get(Axis2D::Horizontal), &1_usize);
        assert_eq!(coord.get_mut(Axis2D::Vertical), &mut 0_usize);
        assert_eq!(coord.get_mut(Axis2D::Horizontal), &mut 1_usize);

        assert_eq!(coord.as_tuple(), (&0_usize, &1_usize));
        assert_eq!(coord.as_tuple_mut(), (&mut 0_usize, &mut 1_usize));
        assert_eq!(coord.as_array(), [&0_usize, &1_usize]);
        assert_eq!(coord.as_array_mut(), [&mut 0_usize, &mut 1_usize]);
        assert_eq!(coord.into_tuple(), (0_usize, 1_usize));
        assert_eq!(coord.into_array(), [0_usize, 1_usize]);
        assert_eq!(coord.into_tuple_const(), (0_usize, 1_usize));
        assert_eq!(coord.into_array_const(), [0_usize, 1_usize]);

        assert_eq!(coord[0], 0_usize);
        assert_eq!(coord[1], 1_usize);

        coord[0] = 2_usize;
        assert_eq!(coord[0], 2_usize);
        coord[1] = 4_usize;
        assert_eq!(coord[1], 4_usize);
        coord[Axis2D::Vertical] = 3_usize;
        assert_eq!(coord[Axis2D::Vertical], 3_usize);
        coord[Axis2D::Horizontal] = 6_usize;
        assert_eq!(coord[Axis2D::Horizontal], 6_usize);
    }

    #[test]
    fn coord_conversion() {
        let coord = Coordinate::new(0_usize, 1_usize);

        assert_eq!(Coordinate::from((0_usize, 1_usize)), coord);
        assert_eq!(
            <Coordinate<usize> as Into<(usize, usize)>>::into(coord),
            (0, 1)
        );
        assert_eq!(Coordinate::from([0, 1]), coord);
        assert_eq!(<Coordinate<usize> as Into<[usize; 2]>>::into(coord), [0, 1]);

        let array = [0_usize, 1_usize];
        assert_eq!(<Coordinate<usize> as From<&[usize]>>::from(&array), coord);
        assert_eq!(Coordinate::from(array.to_vec()), coord);
        let array = [4_usize];
        assert_eq!(
            <Coordinate<usize> as From<&[usize]>>::from(&array),
            Coordinate::new(4_usize, 0_usize)
        );
        assert_eq!(
            Coordinate::from(array.to_vec()),
            Coordinate::new(4_usize, 0_usize)
        );
    }

    #[test]
    fn coord_math() {
        let mut c1 = Coordinate::new(3_i32, -5_i32);
        let c2 = Coordinate::new(1_i32, 0_i32);
        let c3 = Coordinate::new(4_i32, -5_i32);
        c1 += c2;

        assert_eq!(c1, c3);

        c1 -= c2;

        assert_eq!(c1, Coordinate::new(3_i32, -5_i32));

        assert_eq!(c1 + c2, c3);
        assert_eq!(-c1 - c2, -c3);

        assert!(Coordinate::<i32>::zero().is_zero());
        assert_eq!(Coordinate::zero(), Coordinate::new(0_i32, 0_i32));
        assert!(Coordinate::<f64>::zero().is_zero());
        assert!(Coordinate::<PositiveFloat>::zero().is_zero());
    }

    #[test]
    fn fmt() {
        assert_eq!(Coordinate::new(4_u32, 1053_u32).to_string(), "[4, 1053]");
        assert_eq!(
            format!("{:o}", Coordinate::new(0o1241_u16, 0o6761_u16)),
            "[1241, 6761]"
        );
        assert_eq!(
            format!("{:x}", Coordinate::new(0x21_u8, 0xf6_u8)),
            "[21, f6]"
        );
        assert_eq!(
            format!("{:X}", Coordinate::new(0x21_u8, 0xf6_u8)),
            "[21, F6]"
        );

        let x = 1_i32;
        let y = 2_i32;
        let c = Coordinate::new(&x, &y);
        assert_eq!(format!("{c:p}"), format!("[{:p}, {:p}]", &x, &y));

        assert_eq!(
            format!("{:b}", Coordinate::new(0b_0011_1111, 0b_1100_0000_u8)),
            "[111111, 11000000]"
        );
        assert_eq!(
            format!("{:e}", Coordinate::new(1.4e+5_f64, 6.7e-6_f64)),
            "[1.4e5, 6.7e-6]"
        );
        assert_eq!(
            format!("{:E}", Coordinate::new(1.4E+5_f64, 6.7E-6_f64)),
            "[1.4E5, 6.7E-6]"
        );
        assert_eq!(
            format!("{:.1}", Coordinate::new(1.44_f64, 6.78_f64)),
            "[1.4, 6.8]"
        );
    }
}
