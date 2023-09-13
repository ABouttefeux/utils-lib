use std::{
    fmt::{self, Display},
    hash::Hash,
};

pub trait GetterAttributeOption: Hash {}

//TODO name
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MutableGetterAttributeOption {
    Visibility,
    IdentOption,
}

impl GetterAttributeOption for MutableGetterAttributeOption {}

impl Display for MutableGetterAttributeOption {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visibility => write!(f, "visibility"),
            Self::IdentOption => write!(f, "name"),
        }
    }
}

//TODO name
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ImmutableGetterAttributeOption {
    MutableOption(MutableGetterAttributeOption),
    ConstTy,
    GetterTy,
    SelfTy,
}

impl GetterAttributeOption for ImmutableGetterAttributeOption {}

impl Display for ImmutableGetterAttributeOption {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MutableOption(option) => write!(f, "{option}"),
            Self::ConstTy => write!(f, "const"),
            Self::GetterTy => write!(f, "getter type"),
            Self::SelfTy => write!(f, "self"),
        }
    }
}

impl From<MutableGetterAttributeOption> for ImmutableGetterAttributeOption {
    #[inline]
    fn from(value: MutableGetterAttributeOption) -> Self {
        Self::MutableOption(value)
    }
}
