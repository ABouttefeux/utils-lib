use proc_macro2::{Ident, Span};
use syn::Field;

use super::attribute_option::AttributeOptionParseUtils;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub struct IdentOption {
    name: Option<Ident>,
}

impl IdentOption {
    /// Path string for the name option
    const NAME_PATH: &'static str = "name";

    #[inline]
    #[must_use]
    const fn new(name: Option<Ident>) -> Self {
        Self { name }
    }

    #[inline]
    #[must_use]
    fn ident<'a>(&'a self, field: &'a Field) -> Option<&'a Ident> {
        self.name.as_ref().or(field.ident.as_ref())
    }

    #[inline]
    #[must_use]
    pub fn name<'a>(&'a self, field: &'a Field) -> Option<&'a Ident> {
        self.ident(field)
    }

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

impl AttributeOptionParseUtils for IdentOption {
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
