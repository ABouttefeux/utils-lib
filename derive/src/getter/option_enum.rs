//! Contains the enums for the list of options [`MutableOptionList`] and [`ImmutableOptionList`]
//! and the trait [`OptionList`]

use std::{
    fmt::{self, Display},
    hash::Hash,
};

/// Trait for common code for listing option:
/// [`MutableOptionList`] and [`ImmutableOptionList`].
pub trait OptionList {}

/// List option for [`super::option::MutableGetterOption`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MutableOptionList {
    /// Visibility
    Visibility,
    /// name
    IdentOption,
}

impl OptionList for MutableOptionList {}

impl Display for MutableOptionList {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility => write!(f, "visibility"),
            Self::IdentOption => write!(f, "name"),
        }
    }
}

/// List option for [`super::option::ImmutableGetterOption`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ImmutableOptionList {
    /// Common option with mut getter:
    /// - name
    /// - visibility
    MutableOption(MutableOptionList),
    /// if the function is constant or not
    ConstTy,
    /// if the getter is by ref, value or clone
    GetterTy,
    /// if the self value is by ref or moved
    SelfTy,
}

impl OptionList for ImmutableOptionList {}

impl Display for ImmutableOptionList {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MutableOption(option) => write!(f, "{option}"),
            Self::ConstTy => write!(f, "const"),
            Self::GetterTy => write!(f, "getter type"),
            Self::SelfTy => write!(f, "self type"),
        }
    }
}

impl From<MutableOptionList> for ImmutableOptionList {
    #[inline]
    fn from(value: MutableOptionList) -> Self {
        Self::MutableOption(value)
    }
}
