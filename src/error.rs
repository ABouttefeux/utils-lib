//! Contains the errors definitions.

use std::{
    error::Error,
    fmt::{self, Display},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The error equivalent of getting a [`None`] on an [`Option`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NoneError;

impl Display for NoneError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the option had a none value")
    }
}

impl From<()> for NoneError {
    #[inline]
    fn from((): ()) -> Self {
        Self
    }
}

impl From<NoneError> for () {
    #[inline]
    fn from(_error: NoneError) -> Self {}
}

impl Error for NoneError {}
