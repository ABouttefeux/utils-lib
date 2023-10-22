//! Contains [`CoordinateIterator`] an iterators for [`Coordinate`].
//! It is called by [`Coordinate::into_iter`], [`Coordinate::iter`]
//! and [`Coordinate::iter_mut`].

use std::iter::FusedIterator;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Axis2D, Coordinate};

/// [`Iterator`] on a coordinate [`Coordinate`]. It is the type return by [`Coordinate::into_iter`]
/// (and [`Coordinate::iter`] and [`Coordinate::iter_mut`] thought behind implicit type) .
///
/// Also implement [`DoubleEndedIterator`], [`FusedIterator`] and [`ExactSizeIterator`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)] // it should not be copy as it is an iterator (clippy::copy_iterator)
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
            coord: coord.into(),
            front: Some(Axis2D::AXIS[0]),
            back: None,
        }
    }

    /// converts a `&CoordinateIterator<T>` into a `CoordinateIterator<&T>`.
    #[inline]
    pub const fn as_ref(&self) -> CoordinateIterator<&T> {
        CoordinateIterator {
            coord: Coordinate::new(self.coord.x.as_ref(), self.coord.y.as_ref()),
            front: self.front,
            back: self.back,
        }
    }

    /// converts a `&mut CoordinateIterator<T>` into a `CoordinateIterator<&mut T>`.
    #[inline]
    pub fn as_mut(&mut self) -> CoordinateIterator<&mut T> {
        CoordinateIterator {
            coord: Coordinate::new(self.coord.x.as_mut(), self.coord.y.as_mut()),
            front: self.front,
            back: self.back,
        }
    }
}

/// Used for [`CoordinateIterator::new`].
impl<T> From<Coordinate<T>> for Coordinate<Option<T>> {
    #[inline]
    fn from(coord: Coordinate<T>) -> Self {
        Self::new(Some(coord.x), Some(coord.y))
    }
}

// /// implemented for possible use in [`CoordinateIterator`]
// impl<T> From<Coordinate<T>> for Coordinate<MaybeUninit<T>> {
//     #[inline]
//     fn from(coord: Coordinate<T>) -> Self {
//         Self::new(MaybeUninit::new(coord.x), MaybeUninit::new(coord.y))
//     }
// }

/// Same as [`CoordinateIterator::as_ref`].
impl<'a, T> From<&'a CoordinateIterator<T>> for CoordinateIterator<&'a T> {
    #[inline]
    fn from(value: &'a CoordinateIterator<T>) -> Self {
        value.as_ref()
    }
}

/// Same as [`CoordinateIterator::as_mut`].
impl<'a, T> From<&'a mut CoordinateIterator<T>> for CoordinateIterator<&'a mut T> {
    #[inline]
    fn from(value: &'a mut CoordinateIterator<T>) -> Self {
        value.as_mut()
    }
}

/// Create a new iterator with of a [`Coordinate`] with default element
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

/// equivalent as calling [`Coordinate::into_iter`].
impl<T> From<Coordinate<T>> for CoordinateIterator<T> {
    #[inline]
    fn from(value: Coordinate<T>) -> Self {
        value.into_iter()
    }
}

/// equivalent as calling `<&Coordinate>::into_iter`.
impl<'a, T> From<&'a Coordinate<T>> for CoordinateIterator<&'a T> {
    #[inline]
    fn from(value: &'a Coordinate<T>) -> Self {
        value.into_iter()
    }
}

/// equivalent as calling `<&mut Coordinate>::into_iter`.
impl<'a, T> From<&'a mut Coordinate<T>> for CoordinateIterator<&'a mut T> {
    #[inline]
    fn from(value: &'a mut Coordinate<T>) -> Self {
        value.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::{Coordinate, CoordinateIterator};

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

        let mut iter = CoordinateIterator::<String>::default();
        assert_eq!(iter.next(), Some(String::default()));
        assert_eq!(iter.next(), Some(String::default()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn conversion_iter() {
        let c = Coordinate::new(0_usize, 1_usize);
        let mut iter = c.into_iter();

        let mut i_ref = iter.as_ref();
        assert_eq!(i_ref.next(), Some(&0_usize));
        assert_eq!(i_ref.next(), Some(&1_usize));
        assert_eq!(i_ref.next(), None);

        let mut i_mut = iter.as_mut();
        let mut_ref = i_mut.next();
        assert_eq!(mut_ref, Some(&mut 0_usize));
        *mut_ref.expect("it is some") = 4_usize;

        let mut_ref = i_mut.next();
        assert_eq!(mut_ref, Some(&mut 1_usize));
        *mut_ref.expect("it is some") = 5_usize;

        assert_eq!(i_mut.next(), None);

        assert_eq!(iter.next(), Some(4_usize));
        assert_eq!(iter.next(), Some(5_usize));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut c = Coordinate::new(0_usize, 1_usize);

        let iter = Into::<CoordinateIterator<_>>::into(c);
        assert_eq!(iter, c.into_iter());
        let iter = Into::<CoordinateIterator<_>>::into(&c);
        assert_eq!(iter, (&c).into_iter());
        let mut iter = Into::<CoordinateIterator<_>>::into(&mut c);
        assert_eq!(iter.next(), Some(&mut 0));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn conversion_coord() {
        let coord = Coordinate::new(0_i32, 1_i32);
        let c_opt = Into::<Coordinate<Option<i32>>>::into(coord);
        assert_eq!(c_opt, Coordinate::new(Some(0_i32), Some(1_i32)));

        // let c_maybe_uninit = Into::<Coordinate<MaybeUninit<i32>>>::into(coord);
        // let coord_check = Coordinate::new(MaybeUninit::new(0_i32), MaybeUninit::new(1_i32));

        // for (el, check) in c_maybe_uninit.into_iter().zip(coord_check.into_iter()) {
        //     assert_eq!(
        //         // SAFETY: this should be safe
        //         unsafe { el.assume_init() },
        //         // SAFETY: this is safe we use MaybeUninit::new
        //         unsafe { check.assume_init() }
        //     );
        // }
    }
}
