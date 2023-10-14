//! Contains [`SelfTy`]

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

use super::attribute_option::ParseOptionUtils;

/// TODO
///
/// Accepted value:
/// - `self` or `&self`
/// - `self = "..."`, `self_type = "..."`, `self_ty = "..."`
/// - `self(...)`, `self_type(...)`, `self_ty(...)`
/// where ... is `ref`, `value`, `copy`, `move`, `self` or `&self`
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
    /// It is recommended only for Self type that implements [`Copy`] and is smaller than a word.
    Value,
}

impl SelfTy {
    /// add a `&` symbol if it is a [`Self::Ref`] otherwise add nothing
    fn quote(self) -> TokenStream2 {
        match self {
            Self::Ref => quote!(&),
            Self::Value => quote!(),
        }
    }
}

impl ParseOptionUtils for SelfTy {
    fn parse_option_from_str(_path: &str) -> Option<Self> {
        // non working self, &self syntax
        // if path == "self" {
        //     Some(Self::Value)
        // } else if path == "&self" {
        //     Some(Self::Ref)
        // } else {
        //     None
        // }
        None
    }

    fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
        Self::parse_option_from_str(path).or_else(|| {
            if path == "value" || path == "copy" || path == "move" {
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

impl ToTokens for SelfTy {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(self.quote());
    }
}
