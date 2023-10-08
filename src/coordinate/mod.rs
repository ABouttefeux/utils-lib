//! Module containing [`Coordinate`] a 2d coordinate and [`Axis2D`] an enumeration
//! of the x and y axis.

mod iterator;

use std::{
    iter::FusedIterator,
    ops::{Add, AddAssign, Index, IndexMut, Neg, Not, Sub, SubAssign},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
pub use self::iterator::CoordinateIterator;
use crate::{error::NoneError, number::abs_diff};

/// Represent the Axis in 2 dimensions. It can be either in the `x` direction i.e. [`Self::Vertical`]
/// or the `y` direction, i.e. [`Self::Horizontal`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(clippy::exhaustive_enums)] // reason = "no more variant possible"
pub enum Axis2D {
    /// X axis
    #[default]
    Vertical,
    /// Y Axis
    Horizontal,
}

impl Axis2D {
    /// All the possible axis
    pub const AXIS: [Self; 2] = [Self::Vertical, Self::Horizontal];

    /// Convert an index into an [`Axis2D`]
    ///
    /// # Example
    /// ```
    /// use utils_lib::coordinate::Axis2D;
    ///
    /// assert_eq!(Axis2D::from_index(0), Some(Axis2D::Vertical));
    /// assert_eq!(Axis2D::from_index(1), Some(Axis2D::Horizontal));
    /// assert_eq!(Axis2D::from_index(2), None);
    /// assert_eq!(Axis2D::from_index(3), None);
    /// //...
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Vertical),
            1 => Some(Self::Horizontal),
            _ => None,
        }
    }

    /// Convert an [`Axis2D`] into an index
    ///
    /// # Example
    /// ```
    /// use utils_lib::coordinate::Axis2D;
    ///
    /// assert_eq!(Axis2D::Vertical.to_index(), 0);
    /// assert_eq!(Axis2D::Horizontal.to_index(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_index(self) -> usize {
        match self {
            Self::Vertical => 0,
            Self::Horizontal => 1,
        }
    }

    /// Convert an [`Axis2D`] as an index
    ///
    /// # Example
    /// ```
    /// use utils_lib::coordinate::Axis2D;
    ///
    /// assert_eq!(Axis2D::Vertical.as_index(), &0);
    /// assert_eq!(Axis2D::Horizontal.as_index(), &1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_index(self) -> &'static usize {
        match self {
            Self::Vertical => &0,
            Self::Horizontal => &1,
        }
    }

    /// Get the perpendicular axis
    ///
    /// # Example
    /// ```
    /// use utils_lib::coordinate::Axis2D;
    ///
    /// assert_eq!(Axis2D::Vertical.perpendicular(), Axis2D::Horizontal);
    /// assert_eq!(Axis2D::Horizontal.perpendicular(), Axis2D::Vertical);
    /// ```
    #[inline]
    #[must_use]
    pub const fn perpendicular(self) -> Self {
        match self {
            Self::Vertical => Self::Horizontal,
            Self::Horizontal => Self::Vertical,
        }
    }

    /// Convert an [`Axis2D`] into a cardinal direction in the form of a [`Coordinate::<usize>`]
    ///
    /// # Example
    /// ```
    /// use utils_lib::coordinate::{Axis2D, Coordinate};
    ///
    /// assert_eq!(Axis2D::Vertical.coordinate_usize(), Coordinate::new(1, 0));
    /// assert_eq!(Axis2D::Horizontal.coordinate_usize(), Coordinate::new(0, 1));
    /// ```
    #[inline]
    #[must_use]
    pub const fn coordinate_usize(self) -> Coordinate<usize> {
        match self {
            Self::Vertical => Coordinate::new(1, 0),
            Self::Horizontal => Coordinate::new(0, 1),
        }
    }
}

/// private functions for iterator
impl Axis2D {
    /// gives the next index when use to index the front of [`CoordinateIterator`]
    const fn next(self) -> Option<Self> {
        match self {
            Self::Vertical => Some(Self::Horizontal),
            Self::Horizontal => None,
        }
    }

    /// gives the previous index when use to index the back of [`CoordinateIterator`]
    const fn next_back(val: Option<Self>) -> Option<Self> {
        match val {
            Some(Self::Vertical) => None,
            Some(Self::Horizontal) => Some(Self::Vertical),
            None => Some(Self::Horizontal),
        }
    }

    /// gives the size hint for the index that should be used as `back - front`
    const fn size_hint(val: Option<Self>) -> usize {
        match val {
            Some(axis) => axis.to_index(),
            None => 2_usize,
        }
    }
}

impl Not for Axis2D {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        self.perpendicular()
    }
}

impl From<Axis2D> for usize {
    #[inline]
    fn from(value: Axis2D) -> Self {
        value.to_index()
    }
}

impl From<Axis2D> for Coordinate<usize> {
    #[inline]
    fn from(value: Axis2D) -> Self {
        value.coordinate_usize()
    }
}

impl TryFrom<usize> for Axis2D {
    // TODO
    type Error = NoneError;

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_index(value).ok_or(NoneError)
    }
}

impl AsRef<usize> for Axis2D {
    #[inline]
    fn as_ref(&self) -> &usize {
        self.as_index()
    }
}

// TODO conversion Coord

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

#[cfg(test)]
mod test {

    use super::{Axis2D, Coordinate};
    use crate::error::NoneError;

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
    fn axis_2d_iter() {
        assert_eq!(Axis2D::Vertical.next(), Some(Axis2D::Horizontal));
        assert_eq!(Axis2D::Horizontal.next(), None);

        assert_eq!(Axis2D::next_back(None), Some(Axis2D::Horizontal));
        assert_eq!(
            Axis2D::next_back(Some(Axis2D::Horizontal)),
            Some(Axis2D::Vertical)
        );
        assert_eq!(Axis2D::next_back(Some(Axis2D::Vertical)), None);
    }
}
