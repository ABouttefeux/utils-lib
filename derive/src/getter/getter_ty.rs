//! Module containing [`GetterTy`]

use std::fmt::{self, Display};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::attribute_option::ParseOptionUtils;

// TODO refactoring less code duplication

/// Getter type. By default it is by reference (and it is in most case).
/// However it can useful to the [`GetterTy::Copy`] type for small copy type
/// as it can be more optimize if they are smaller than a [`usize`].
/// There also the clone type. I don't see a lot of use but it is there if you want.
///
/// Accepted value:
/// - `by_ref`, `by_value`, `by_copy`, `by_clone`, `copy`, `clone`
/// - `getter_ty = "..."`, `getter_type = "..."`
/// - `getter_ty("...")`, `getter_type("...")`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub enum GetterTy {
    /// to get the field by copy for example
    /// ```
    /// # struct S {
    /// #   field: u32,
    /// # }
    /// #
    /// # impl S {
    /// fn field(&self) -> u32 {
    ///     self.field
    /// }
    /// # }
    /// ```
    /// works only for type that implements [`Copy`].
    Copy,
    /// to get the field by a clone for example
    /// ```
    /// # struct S {
    /// #   field: String,
    /// # }
    /// #
    /// # impl S {
    /// fn field(&self) -> String {
    ///     self.field.clone()
    /// }
    /// # }
    /// ```
    /// works only for type that implements [`Clone`].
    Clone,
    /// to get the field by reference for example
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
}

impl GetterTy {
    /// Get the quote for start of the function implementation
    #[must_use]
    #[inline]
    pub fn prefix_quote(self) -> TokenStream2 {
        match self {
            Self::Ref => quote! {&},
            Self::Clone | Self::Copy => quote! {},
        }
    }

    /// Get the quote for end of the function implementation
    #[must_use]
    #[inline]
    pub fn suffix_quote(self) -> TokenStream2 {
        match self {
            Self::Clone => quote! {.clone()},
            Self::Copy | Self::Ref => quote! {},
        }
    }

    /// Parse the option from a string
    #[must_use]
    #[inline]
    fn parse_string(path: &str) -> Option<Self> {
        match path {
            "by_ref" | "by ref" => Some(Self::Ref),
            "by_value" | "by_copy" | "copy" | "Copy" => Some(Self::Copy),
            "by_clone" | "clone" | "Clone" => Some(Self::Clone),
            _ => None,
        }
    }

    /// Get the left hand value accepted in the parsing of the option
    #[must_use]
    #[inline]
    fn left_hand_path_accepted_self(path: &str) -> bool {
        path == "getter_ty" || path == "getter_type" || path == "Getter_ty" || path == "Getter_type"
    }
}

// impl AttributeOptionParse for GetterTy {
//     #[inline]
//     fn parse_option(option: &Meta) -> Option<Self> {
//         match option {
//             Meta::Path(path) => Self::parse_path(path),
//             Meta::NameValue(name_value) => {
//                 //
//                 let ident = name_value.path.get_ident()?;
//                 if Self::left_hand_path_accepted(&ident.to_string()) {
//                     if let Expr::Lit(ExprLit {
//                         lit: Lit::Str(ref lit_string),
//                         ..
//                     }) = &name_value.value
//                     {
//                         Self::parse_string(&lit_string.value())
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 }
//             }
//             Meta::List(meta_list) => {
//                 let ident = meta_list.path.get_ident()?;
//                 if Self::left_hand_path_accepted(&ident.to_string()) {
//                     meta_list
//                         .parse_args::<Ident>()
//                         .map_or(None, |ident| Self::parse_string(&ident.to_string()))
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
// }

impl ParseOptionUtils for GetterTy {
    #[inline]
    fn parse_option_from_str(path: &str) -> Option<Self> {
        Self::parse_string(path)
    }

    #[inline]
    fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
        Self::parse_option_from_str(path)
    }

    #[inline]
    fn left_hand_path_accepted(path: &str) -> bool {
        Self::left_hand_path_accepted_self(path)
    }
}

impl Display for GetterTy {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ref => write!(f, "reference"),
            Self::Copy => write!(f, "copied value"),
            Self::Clone => write!(f, "cloned value"),
        }
    }
}
