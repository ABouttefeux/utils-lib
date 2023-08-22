use std::{
    error::Error,
    fmt::{self, Display},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

impl Error for NoneError {}
