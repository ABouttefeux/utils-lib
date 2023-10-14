//! Contains [`Visibility`]

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Path;

use super::attribute_option::ParseOptionUtils;

/// Visibility option
///
/// ! #[get(pub)]!  or `#[get(visibility = pub)]`
///
/// accepted option :
/// - pub, public, crate, pub(...), private,
/// - Visibility = "..."
/// - Visibility("...")
#[derive(Clone, Default)]
pub enum Visibility {
    /// Public, pub modifier like `pub fn`.
    Public,
    #[default]
    /// Private, no modifier like `fn`.
    /// Default value
    Private,
    /// Crate visibility like `pub(crate) fn` or `pub(super) fn`
    Crate(Option<Path>),
}

impl Visibility {
    /// string for left hand value for visibility.
    /// visibility =
    const VISIBILITY_LEFT_HAND: &'static str = "visibility";

    // TODO
    /// Try parse a a [`Visibility`] from a `&str` as the modifier
    #[inline]
    fn visibility_from_path_str(string: &str) -> Option<Self> {
        if string == "pub" || string == "public" || string == "Public" || string == "Pub" {
            return Some(Self::Public);
        } else if string == "crate" || string == "Crate" {
            return Some(Self::Crate(None));
        } else if string == "private" || string == "Private" {
            return Some(Self::Private);
        } else if let Some((left, right)) = string.split_once('(') {
            if left == "pub" {
                if let Some(vis_path) = right.strip_suffix(')') {
                    return Some(Self::Crate(Some(syn::parse_str(vis_path).ok()?)));
                }
            }
        }

        None
    }
}

impl Visibility {
    /// create a token a quote of the visibility
    fn quote(&self) -> TokenStream2 {
        match self {
            Self::Private => quote!(),
            Self::Public => quote!(pub),
            Self::Crate(path) => path
                .as_ref()
                .map_or_else(|| quote!(pub(crate)), |path| quote!(pub(#path))),
        }
    }
}

impl ParseOptionUtils for Visibility {
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
        path == Self::VISIBILITY_LEFT_HAND || path == "Visibility"
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(self.quote());
    }
}
