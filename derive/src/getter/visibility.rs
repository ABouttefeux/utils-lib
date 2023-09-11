use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Field;

use super::attribute_option::{AttributeOptionParseUtils, ToCode};

/// Visibility option
///
/// ! #[get(pub)]!  or `#[get(visibility = pub)]`
///
/// accepted option :
/// - pub, public, crate, pub(...), private,
/// - Visibility = "..."
/// - Visibility("...")
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum Visibility {
    /// Public, pub modifier like `pub fn`.
    Public,
    #[default]
    /// Private, no modifier like `fn`.
    /// Default value
    Private,
    /// Crate visibility like `pub(crate) fn` or `pub(super) fn`
    Crate(Option<String>),
}

impl Visibility {
    /// string for left hand value for visibility.
    /// visibility =
    const VISIBILITY_LEFT_HAND: &'static str = "visibility";

    // /// Try to parse a [`Visibility`] option from a [`Meta`]
    // #[inline]
    // pub fn visibility_option(option: &Meta) -> Option<Self> {
    //     match option {
    //         Meta::Path(path) => Self::visibility_from_path(path),
    //         Meta::NameValue(name_value) => Self::visibility_from_name_value(name_value),
    //         // FIX ME
    //         Meta::List(meta_list) => {
    //             if meta_list.path.is_ident(Self::VISIBILITY_LEFT_HAND) {
    //                 meta_list.parse_args::<Ident>().map_or(None, |ident| {
    //                     Self::visibility_from_path_str(&ident.to_string())
    //                 })
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    // }

    // /// Try parse a [`Visibility`] from a [`Path`] as the modifier
    // #[inline]
    // fn visibility_from_path(path: &Path) -> Option<Self> {
    //     path.get_ident()
    //         .and_then(|ident| Self::visibility_from_path_str(&ident.to_string()))
    // }

    /// Try parse a a [`Visibility`] from a `&str` as the modifier
    #[inline]
    fn visibility_from_path_str(string: &str) -> Option<Self> {
        if string == "pub" || string == "public" {
            return Some(Self::Public);
        } else if string == "crate" {
            return Some(Self::Crate(None));
        } else if string == "private" {
            return Some(Self::Private);
        } else if let Some((left, right)) = string.split_once('(') {
            if left == "pub" {
                if let Some(vis_path) = right.strip_suffix(')') {
                    return Some(Self::Crate(Some(vis_path.to_owned())));
                }
            }
        }

        None
    }

    // /// Try parse a a [`Visibility`] from a [`MetaNameValue`]
    // #[inline]
    // fn visibility_from_name_value(name_value: &MetaNameValue) -> Option<Self> {
    //     if name_value.path.is_ident(Self::VISIBILITY_LEFT_HAND) {
    //         if let Expr::Lit(ExprLit {
    //             lit: Lit::Str(ref lit_string),
    //             ..
    //         }) = &name_value.value
    //         {
    //             Self::visibility_from_path_str(&lit_string.value())
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
}

impl AttributeOptionParseUtils for Visibility {
    #[inline]
    fn parse_option_from_str(path: &str) -> Option<Self> {
        Self::visibility_from_path_str(path)
    }

    #[inline]
    fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
        Self::parse_option_from_str(path)
    }

    #[inline]
    fn left_hand_path_accepted(path: &str) -> bool {
        path == Self::VISIBILITY_LEFT_HAND
    }
}

impl ToCode for Visibility {
    fn to_code(&self, _: &Field) -> TokenStream2 {
        match self {
            Self::Private => TokenStream2::new(),
            Self::Public => quote! {pub},
            Self::Crate(opt_str) => {
                let opt_str = opt_str.as_ref().map_or("crate", |s| s.as_str());

                quote! {pub(#opt_str)}
            }
        }
    }
}
