//! contains [`Axis2D`] an enumeration the of the x and y axis.

use std::ops::Not;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Coordinate;
use crate::error::NoneError;

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
    /// gives the next index when use to index the front of [`super::CoordinateIterator`]
    pub(super) const fn next(self) -> Option<Self> {
        match self {
            Self::Vertical => Some(Self::Horizontal),
            Self::Horizontal => None,
        }
    }

    /// gives the previous index when use to index the back of [`super::CoordinateIterator`]
    pub(super) const fn next_back(val: Option<Self>) -> Option<Self> {
        match val {
            Some(Self::Vertical) => None,
            Some(Self::Horizontal) => Some(Self::Vertical),
            None => Some(Self::Horizontal),
        }
    }

    /// gives the size hint for the index that should be used as `back - front`
    pub(super) const fn size_hint(val: Option<Self>) -> usize {
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

#[cfg(test)]
mod test {
    use super::Axis2D;

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
