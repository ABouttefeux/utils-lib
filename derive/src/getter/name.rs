//! Contains [`FunctionName`]

use macro_utils::field::FieldName;
use proc_macro2::{Ident, Span};

use super::attribute_option::ParseOptionUtils;

// TODO rename to name

/// optional name of the getter
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub struct FunctionName {
    /// Wrapped ident value
    name: Option<Ident>,
}

impl FunctionName {
    /// Path string for the name option
    const NAME_PATH: &'static str = "name";

    /// wrap a new [`Option::<Ident>`] into a new [`Self`]
    #[inline]
    #[must_use]
    const fn new(name: Option<Ident>) -> Self {
        Self { name }
    }

    /// Get the getter function name as an [`Ident`]. see [`Self::name`]
    #[must_use]
    fn ident<'a>(&'a self, field: &'a FieldName) -> Option<&'a Ident> {
        self.name.as_ref().or_else(|| field.require_ident())
    }

    // cspell: ignore identless
    /// Get the getter function name as an [`Ident`].
    ///
    /// Return [`None`] if the field is identless and the name option is left unset.
    #[must_use]
    pub fn name<'a>(&'a self, field: &'a FieldName) -> Option<&'a Ident> {
        self.ident(field)
    }

    /// Get the mut getter function name as an [`Ident`].
    ///
    /// Return [`None`] if the field is identless and the name option is left unset.
    #[must_use]
    pub fn name_mut(&self, field: &FieldName) -> Option<Ident> {
        self.name.clone().or_else(|| {
            field
                .require_ident()
                .map(|ident| Ident::new(&format!("{ident}_mut"), Span::call_site()))
        })
    }
}

impl ParseOptionUtils for FunctionName {
    #[inline]
    fn parse_option_from_str(_path: &str) -> Option<Self> {
        None
    }

    fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
        Some(Self::new(Some(Ident::new(path, Span::call_site()))))
    }

    #[inline]
    fn left_hand_path_accepted(path: &str) -> bool {
        path == Self::NAME_PATH
    }
}
