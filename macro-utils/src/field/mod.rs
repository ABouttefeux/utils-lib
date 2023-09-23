//! Contains [`Field`]

use std::fmt::{self, Display};

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{Index, Type};

/// Contain a [`syn::Field`] and an index that track the index of the field to
/// getter working getter on tuple structure
#[derive(Clone)]
pub struct Field {
    /// the syn field
    pub field: syn::Field,
    /// the position of the field. Mostly for tuple struct
    pub index: usize,
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

/// Represent the way to access a field. Either with [`Self::Ident`] or [`Self::Index`]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::exhaustive_enums)]
#[derive(Clone)]
pub enum FieldName {
    /// the field is accessed with an ident as name field
    Ident(Ident),
    /// the field is accessed with an index as a tuple or uname field
    Index(Index),
}

impl FieldName {
    /// Get the way to access the field from a [`Field`].
    /// It clones only `field.ident` and therefore less resource intensive
    /// than cloning the whole field in [`Self::from_field`] using `Self::from_field(field.clone())`.
    #[must_use]
    pub fn from_field_ref(field: &Field) -> Self {
        Self::from_field_part(field.field().ident.clone(), field.index())
    }

    /// Get the way to access the field from a [`Field`].
    /// If you want to reuse the field afterward use [`Self::from_field_ref`] instead.
    #[must_use]
    pub fn from_field(field: Field) -> Self {
        Self::from_field_part(field.field.ident, field.index)
    }

    /// Create a new self from some moved part of a [`Field`] (the ident and the index).
    #[must_use]
    fn from_field_part(opt_ident: Option<Ident>, index: usize) -> Self {
        opt_ident.map_or_else(|| index.into(), Into::into)
    }

    /// Return [`Some`] on an [`Ident`] and [`None`] on an [`Index`]
    #[must_use]
    pub const fn require_ident(&self) -> Option<&Ident> {
        match self {
            Self::Ident(ref ident) => Some(ident),
            Self::Index(_) => None,
        }
    }
}

impl From<Ident> for FieldName {
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<Index> for FieldName {
    fn from(value: Index) -> Self {
        Self::Index(value)
    }
}

impl From<usize> for FieldName {
    fn from(value: usize) -> Self {
        Index::from(value).into()
    }
}

impl ToTokens for FieldName {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Ident(ref ident) => ident.to_tokens(tokens),
            Self::Index(ref index) => index.to_tokens(tokens),
        }
    }
}

impl Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(ref ident) => write!(f, "{ident}"),
            Self::Index(ref index) => write!(f, "{}", index.index),
        }
    }
}

/// Contain the [`FieldName`] and [`Type`] of a field
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct FieldInformation {
    /// the way to access the field
    pub field_name: FieldName,
    /// The type of the field
    pub ty: Type,
}

impl FieldInformation {
    /// Create a [`FieldInformation`] from a [`Field`].
    #[must_use]
    pub fn from_field(field: Field) -> Self {
        Self {
            field_name: FieldName::from_field_part(field.field.ident, field.index),
            ty: field.field.ty,
        }
    }

    /// Getter on the field name.
    #[must_use]
    pub const fn field_name(&self) -> &FieldName {
        &self.field_name
    }

    /// Getter on the [`Type`] of the field.
    #[must_use]
    pub const fn ty(&self) -> &Type {
        &self.ty
    }
}
