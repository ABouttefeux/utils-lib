//TODO name
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MutableGetterAttributeOption {
    Visibility,
    IdentOption,
}

//TODO name
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ImmutableGetterAttributeOption {
    MutableOption(MutableGetterAttributeOption),
    ConstTy,
    GetterTy,
    SelfTy,
}

impl From<MutableGetterAttributeOption> for ImmutableGetterAttributeOption {
    #[inline]
    fn from(value: MutableGetterAttributeOption) -> Self {
        Self::MutableOption(value)
    }
}
