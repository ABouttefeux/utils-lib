//! Contains [`SelfTy`]

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Field;

use super::attribute_option::{FieldAttributeOptionParseUtils, ToCode};

/// TODO
///
/// Accepted value:
/// - `self` or `&self`
/// - `self = "..."`, `self_type = "..."`, `self_ty = "..."`
/// - `self(...)`, `self_type(...)`, `self_ty(...)`
/// where ... is `ref`, `value`, `copy`, `self` or `&self`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub enum SelfTy {
    /// TODO
    /// ```
    /// # struct S {
    /// #   field: String,
    /// # }
    /// #
    /// # impl S {
    /// fn field(&self) -> &String {
    ///     &self.field
    /// }
    /// # }
    /// ```
    /// this is the default behavior.
    #[default]
    Ref,
    /// TODO
    /// ```
    /// # #[derive(Copy, Clone)]
    /// # struct S {
    /// #   field: u32,
    /// # }
    /// #
    /// # impl S {
    /// fn field(self) -> u32 {
    ///     self.field
    /// }
    /// # }
    /// ```
    /// works only for Self type that implements [`Copy`].
    Value,
}

impl FieldAttributeOptionParseUtils for SelfTy {
    fn parse_option_from_str(path: &str) -> Option<Self> {
        if path == "self" {
            Some(Self::Value)
        } else if path == "&self" {
            Some(Self::Ref)
        } else {
            None
        }
    }

    fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
        Self::parse_option_from_str(path).or_else(|| {
            if path == "value" || path == "copy" {
                Some(Self::Value)
            } else if path == "ref" {
                Some(Self::Ref)
            } else {
                None
            }
        })
    }

    fn left_hand_path_accepted(path: &str) -> bool {
        path == "self"
            || path == "self_ty"
            || path == "self_type"
            || path == "Self"
            || path == "Self_ty"
            || path == "Self_type"
    }
}

impl ToCode for SelfTy {
    #[inline]
    fn to_code(&self, _field: &Field) -> TokenStream2 {
        match self {
            Self::Ref => quote! {&},
            Self::Value => quote! {},
        }
    }
}
