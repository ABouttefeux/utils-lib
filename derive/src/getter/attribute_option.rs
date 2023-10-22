//! Contains the trait [`ParseOption`] and its helper trait [`ParseOptionUtils`]
//! and [`ToCode`]
//!
//! [`ParseOptionUtils`] is an helper trait used to parse attribute options in
//! `#[get]` and `#[get_mut]` attribute. More precisely if we would like to parse option
//! like `#[get(visibility = "public")]` or just #[get(public)]. we would write
//! ```
//! # trait ParseOptionUtils: Sized {
//! #     fn parse_option_from_str(path: &str) -> Option<Self>;
//! #     fn parse_option_from_str_assignment(path: &str) -> Option<Self>;
//! #     fn left_hand_path_accepted(path: &str) -> bool;
//! # }
//! #[derive(Default)]
//! pub enum Visibility {
//!     /// Public, pub modifier like `pub fn`.
//!     Public,
//!     #[default]
//!     /// Private, no modifier like `fn`.
//!     /// Default value
//!     Private,
//! }
//!
//! impl ParseOptionUtils for Visibility {
//!     // this function look for standalone value like in `#[get(public)]`
//!     fn parse_option_from_str(path: &str) -> Option<Self> {
//!         if path == "public" {
//!             Some(Self::Public)
//!         } else if path == "private" {
//!             Some(Self::Private)
//!         } else {
//!             None
//!         }
//!     }
//!
//!     // this looks for value in assignments or parenthesis like in
//!     // `#[get(visibility(public))]` or `#[get(visibility = "public")]`
//!     fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
//!         Self::parse_option_from_str(path)
//!     }
//!
//!     // this is to determine the left hand side value in our case `visibility`
//!     fn left_hand_path_accepted(path: &str) -> bool {
//!         path == "visibility"
//!     }
//! }
//! ```

// TODO more explanation about the code.

use macro_utils::field::FieldInformation;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Path};

use super::error::{AcceptableParseError, ParseAttributeOptionError, UnacceptableParseError};

// TODO name
// TODO code to avoid duplication for parsing option
// TODO think about error

/// trait for option element that are parsed from [`Meta`]
pub trait ParseOption: Sized {
    /// try to parse the option element from a [`Meta`] return [`Ok`] if the element is valid.
    ///
    /// # Error
    /// see [`ParseAttributeOptionError`]
    fn parse_option(option: &Meta) -> Result<Self, ParseAttributeOptionError>;
}

/// trait for option element that are parsed from [`Meta`] providing default structure
/// to implement [`ParseOption`] more easily
///
/// # Example
///
/// see level module doc [`self`]
pub trait ParseOptionUtils: Sized {
    /// Try parse the option from a string
    #[must_use]
    fn parse_option_from_str(path: &str) -> Option<Self>;

    /// Try parse the option from a string in the case of an assignment
    #[must_use]
    fn parse_option_from_str_assignment(path: &str) -> Option<Self>;

    /// return accepted value for the left hand element of the assignment.
    #[must_use]
    fn left_hand_path_accepted(path: &str) -> bool;

    /// Try parse a Self from a [`Path`] as the modifier
    #[must_use]
    fn parse_from_path(path: &Path) -> Option<Self> {
        path.get_ident()
            .and_then(|ident| Self::parse_from_ident(ident))
    }

    /// Try parse a Self from a [`Ident`] as the modifier
    #[must_use]
    fn parse_from_ident(ident: &Ident) -> Option<Self> {
        Self::parse_option_from_str(&ident.to_string())
    }

    /// Try parse a Self from a [`Ident`] as an assignment
    #[must_use]
    fn parse_from_ident_assignment(ident: &Ident) -> Option<Self> {
        Self::parse_option_from_str_assignment(&ident.to_string())
    }

    /// Try to parse the option element from a [`Meta`] return [`Some`] if the element is valid
    /// [`Err`] otherwise.
    ///
    /// This is meant to be called in [`ParseOption::parse_option`].
    ///
    /// # Error
    /// see [`ParseAttributeOptionError`]
    fn parse_option_utils(option: &Meta) -> Result<Self, ParseAttributeOptionError> {
        match option {
            Meta::Path(path) => Self::parse_from_path(path)
                .ok_or_else(|| AcceptableParseError::PathNotRecognized.into()),
            Meta::NameValue(name_value) => Self::parse_name_value(name_value),
            Meta::List(meta_list) => Self::parse_meta_list(meta_list),
        }
    }

    /// Try parse the rule from a [`MetaNameValue`].
    fn parse_name_value(name_value: &MetaNameValue) -> Result<Self, ParseAttributeOptionError> {
        if Self::left_hand_path_accepted(
            &name_value
                .path
                .get_ident()
                .ok_or(UnacceptableParseError::LeftHandSideValueNotIdent)?
                .to_string(),
        ) {
            let string = get_string_literal(&name_value.value)
                .ok_or(UnacceptableParseError::RightHandNameValueExprNotLitString)?;
            Self::parse_option_from_str_assignment(&string)
                .ok_or_else(|| UnacceptableParseError::RightHandValueInvalid.into())
        } else {
            Err(AcceptableParseError::LeftHandSideValueNotRecognized.into())
        }
    }

    /// Try parse the rule from a [`MetaList`].
    fn parse_meta_list(meta_list: &MetaList) -> Result<Self, ParseAttributeOptionError> {
        if Self::left_hand_path_accepted(
            &meta_list
                .path
                .get_ident()
                .ok_or(UnacceptableParseError::LeftHandSideValueNotIdent)?
                .to_string(),
        ) {
            // FIXE ME
            Self::parse_from_ident_assignment(&meta_list.parse_args::<Ident>()?)
                .ok_or_else(|| UnacceptableParseError::RightHandValueInvalid.into())
        } else {
            Err(AcceptableParseError::LeftHandSideValueNotRecognized.into())
        }
    }
}

/// Get the [`String`] value of a [`Lit::Str`] from [`Expr`] if it were
/// that particular expression. Otherwise returns [`None`].
///
/// It is very specific but it is used to encapsulate code to parse option.
#[must_use]
pub fn get_string_literal(expr: &Expr) -> Option<String> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(ref lit_string),
        ..
    }) = expr
    {
        Some(lit_string.value())
    } else {
        None
    }
}

/// Auto implementation from [`ParseOptionUtils`] to an [`ParseOption`]
impl<T: ParseOptionUtils> ParseOption for T {
    #[inline]
    fn parse_option(option: &Meta) -> Result<Self, ParseAttributeOptionError> {
        Self::parse_option_utils(option)
    }
}

// TODO review
/// Trait to convert an option to actual implementation code
pub trait ToCode {
    /// get the code with the [`FieldInformation`] information
    #[must_use]
    fn to_code(&self, field: &FieldInformation) -> TokenStream2;
}
