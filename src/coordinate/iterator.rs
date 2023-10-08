//! Contains iterators for [`Coordinate`]

use std::iter::FusedIterator;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Axis2D, Coordinate};

/// [`Iterator`] on a coordinate [`Coordinate`]. It is the type return by [`Coordinate::into_iter`]
/// (and [`Coordinate::iter`] and [`Coordinate::iter_mut`] thought behind implicit type) .
///
/// Also implement [`DoubleEndedIterator`], [`FusedIterator`] and [`ExactSizeIterator`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CoordinateIterator<T> {
    /// the storage of the iterator. As an [`Option`] in order to be able to move T and
    /// leave [`None`] behind.
    coord: Coordinate<Option<T>>,
    /// index on the front of the iterator
    front: Option<Axis2D>,
    /// index on the back of the iterator
    back: Option<Axis2D>,
}

impl<T> CoordinateIterator<T> {
    /// Create a new iterator from a [`Coordinate`].
    #[inline]
    pub fn new(coord: Coordinate<T>) -> Self {
        Self {
            coord: Coordinate::new(Some(coord.x), Some(coord.y)),
            front: Some(Axis2D::AXIS[0]),
            back: None,
        }
    }
}

impl<T: Default> Default for CoordinateIterator<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Coordinate::default())
    }
}

impl<T> Iterator for CoordinateIterator<T> {
    type Item = T;

    #[allow(clippy::unwrap_in_result)] // use to do some check
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let front = self.front.expect("front should not be none");
        let return_val = self.coord[front].take();
        debug_assert!(
            return_val.is_some(),
            "the coordinate has already been taken"
        );
        self.front = front.next();
        return_val
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let val = Axis2D::size_hint(self.back) - Axis2D::size_hint(self.front);
        (val, Some(val))
    }
}

impl<T> DoubleEndedIterator for CoordinateIterator<T> {
    #[allow(clippy::unwrap_in_result)] // use to do some check
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back = Axis2D::next_back(self.back);
        let return_val = self.coord[self.back.expect("back should not be none")].take();
        debug_assert!(
            return_val.is_some(),
            "the coordinate has already been taken"
        );
        return_val
    }
}

impl<T> FusedIterator for CoordinateIterator<T> {}

impl<T> ExactSizeIterator for CoordinateIterator<T> {}

impl<T> IntoIterator for Coordinate<T> {
    //type IntoIter = iter::Chain<iter::Once<Self::Item>, iter::Once<Self::Item>>;
    type IntoIter = CoordinateIterator<T>;
    type Item = T;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        //iter::once(self.x).chain(iter::once(self.y))
        CoordinateIterator::new(self)
    }
}

impl<'a, T> IntoIterator for &'a Coordinate<T> {
    type IntoIter = <Coordinate<Self::Item> as IntoIterator>::IntoIter;
    type Item = &'a T;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Coordinate<T> {
    type IntoIter = <Coordinate<Self::Item> as IntoIterator>::IntoIter;
    type Item = &'a mut T;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::Coordinate;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn iter() {
        let mut c = Coordinate::new(1_usize, 2_usize);
        let iter = c.iter().enumerate();
        for (i, el) in iter {
            assert_eq!(i + 1, *el);
        }

        let mut iter = c.into_iter();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));

        let mut iter = c.into_iter();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));

        let mut iter = c.iter_mut();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next_back(), Some(&mut 2));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
}
