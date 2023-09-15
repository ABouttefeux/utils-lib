//! Contains [`IdentOption`]

use proc_macro2::{Ident, Span};
use syn::Field;

use super::attribute_option::FieldAttributeOptionParseUtils;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
/// optional name of the getter
pub struct IdentOption {
    /// Wrapped ident value
    name: Option<Ident>,
}

impl IdentOption {
    /// Path string for the name option
    const NAME_PATH: &'static str = "name";

    /// wrap a new [`Option::<Ident>`] into a new [`Self`]
    #[inline]
    #[must_use]
    const fn new(name: Option<Ident>) -> Self {
        Self { name }
    }

    /// Get the getter function name as an [`Ident`]. see [`Self::name`]
    #[inline]
    #[must_use]
    fn ident<'a>(&'a self, field: &'a Field) -> Option<&'a Ident> {
        self.name.as_ref().or(field.ident.as_ref())
    }

    // cspell: ignore identless
    /// Get the getter function name as an [`Ident`].
    ///
    /// Return [`None`] if the field is identless and the name option is left unset.
    #[inline]
    #[must_use]
    pub fn name<'a>(&'a self, field: &'a Field) -> Option<&'a Ident> {
        self.ident(field)
    }

    /// Get the mut getter function name as an [`Ident`].
    ///
    /// Return [`None`] if the field is identless and the name option is left unset.
    #[inline]
    #[must_use]
    pub fn name_mut(&self, field: &Field) -> Option<Ident> {
        self.name.clone().or_else(|| {
            field
                .ident
                .as_ref()
                .map(|ident| Ident::new(&format!("{ident}_mut"), Span::call_site()))
        })
    }
}

impl FieldAttributeOptionParseUtils for IdentOption {
    #[inline]
    fn parse_option_from_str(_path: &str) -> Option<Self> {
        None
    }

    #[inline]
    fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
        Some(Self::new(Some(Ident::new(path, Span::call_site()))))
    }

    #[inline]
    fn left_hand_path_accepted(path: &str) -> bool {
        path == Self::NAME_PATH
    }
}
