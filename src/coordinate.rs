use std::{
    iter::{self, FusedIterator},
    ops::{Add, AddAssign, Not, Sub, SubAssign},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::number::abs_diff;

// TODO conversion for Axis2D

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
    pub const AXIS: [Self; 2] = [Self::Vertical, Self::Horizontal];

    #[inline]
    #[must_use]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Vertical),
            1 => Some(Self::Horizontal),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn to_index(self) -> usize {
        match self {
            Self::Vertical => 0,
            Self::Horizontal => 1,
        }
    }

    #[inline]
    #[must_use]
    pub const fn as_index(self) -> &'static usize {
        match self {
            Self::Vertical => &0,
            Self::Horizontal => &1,
        }
    }

    #[inline]
    #[must_use]
    pub const fn perpendicular(self) -> Self {
        match self {
            Self::Vertical => Self::Horizontal,
            Self::Horizontal => Self::Vertical,
        }
    }

    #[inline]
    #[must_use]
    pub const fn coordinate_usize(self) -> Coordinate<usize> {
        match self {
            Self::Vertical => Coordinate::new(1, 0),
            Self::Horizontal => Coordinate::new(0, 1),
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

// TODO conversion Coord

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinate<T> {
    x: T,
    y: T,
}

impl<T> Coordinate<T> {
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn x(&self) -> &T {
        &self.x
    }

    #[inline]
    #[must_use]
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    #[inline]
    #[must_use]
    pub const fn y(&self) -> &T {
        &self.y
    }

    #[inline]
    #[must_use]
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    #[inline]
    #[must_use]
    pub const fn get(&self, axis: Axis2D) -> &T {
        match axis {
            Axis2D::Vertical => self.x(),
            Axis2D::Horizontal => self.y(),
        }
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, axis: Axis2D) -> &mut T {
        match axis {
            Axis2D::Vertical => self.x_mut(),
            Axis2D::Horizontal => self.y_mut(),
        }
    }

    // TODO own iterator for ExactSizeIterator
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> + FusedIterator {
        iter::once(self.x()).chain(iter::once(self.y()))
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + FusedIterator {
        iter::once(&mut self.x).chain(iter::once(&mut self.y))
    }

    #[inline]
    #[must_use]
    pub const fn as_tuple(&self) -> (&T, &T) {
        (self.x(), self.y())
    }

    #[inline]
    #[must_use]
    pub fn as_tuple_mut(&mut self) -> (&mut T, &mut T) {
        (&mut self.x, &mut self.y)
    }

    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> [&T; 2] {
        [self.x(), self.y()]
    }

    #[inline]
    #[must_use]
    pub fn as_array_mut(&mut self) -> [&mut T; 2] {
        [&mut self.x, &mut self.y]
    }

    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> Coordinate<&T> {
        Coordinate::new(self.x(), self.y())
    }

    #[inline]
    #[must_use]
    pub fn as_mut(&mut self) -> Coordinate<&mut T> {
        Coordinate::new(&mut self.x, &mut self.y)
    }
}

impl<T> Coordinate<T> {
    #[inline]
    #[must_use]
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }
}

// ~const Drop
impl<T: Copy> Coordinate<T> {
    #[inline]
    #[must_use]
    pub const fn into_tuple_const(self) -> (T, T) {
        (self.x, self.y)
    }

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

impl<T: AddAssign> AddAssign for Coordinate<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
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

impl<T: SubAssign> SubAssign for Coordinate<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
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
