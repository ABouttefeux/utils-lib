//! Contains [`Field`]

/// Contain a [`syn::Field`] and an index that track the index of the field to
/// getter working getter on tuple structure
#[derive(Clone)]
pub struct Field {
    /// the syn field
    field: syn::Field,
    /// the position of the field. Mostly for tuple struct
    index: usize,
}

impl Field {
    /// the constructor
    #[inline]
    #[must_use]
    pub const fn new(field: syn::Field, index: usize) -> Self {
        Self { field, index }
    }

    /// getter of the syn's field
    #[inline]
    #[must_use]
    pub const fn field(&self) -> &syn::Field {
        &self.field
    }

    /// the getter on the index used for tuple struct
    #[inline]
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
}
