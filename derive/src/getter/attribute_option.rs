//! Contains the trait [`ParseOption`] and its helper trait [`ParseOptionUtils`]
//! and [`ToCode`]

use macro_utils::field::FieldInformation;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Path};

use super::error::{AcceptableParseError, ParseAttributeOptionError, UnacceptableParseError};

// TODO name
// TODO code to avoid duplication for parsing option
// TODO think about error

/// trait for option element that are parsed from [`Meta`]
#[allow(clippy::module_name_repetitions)] // TODO
pub trait ParseOption: Sized {
    /// try to parse the option element from a [`Meta`] return [`Ok`] if the element is valid.
    ///
    /// TODO error doc
    fn parse_option(option: &Meta) -> Result<Self, ParseAttributeOptionError>;
}

/// trait for option element that are parsed from [`Meta`] providing default structure
/// to implement [`AttributeOptionParse`] more easily
#[allow(clippy::module_name_repetitions)] // TODO
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

    /// try to parse the option element from a [`Meta`] return [`Some`] if the element is valid
    /// [`None`] otherwise
    fn parse_option_utils(option: &Meta) -> Result<Self, ParseAttributeOptionError> {
        match option {
            Meta::Path(path) => Self::parse_from_path(path)
                .ok_or_else(|| AcceptableParseError::PathNotRecognized.into()),
            Meta::NameValue(name_value) => Self::parse_name_value(name_value),
            Meta::List(meta_list) => Self::parse_meta_list(meta_list),
        }
    }

    /// try parse the rule from a [`MetaNameValue`]
    fn parse_name_value(name_value: &MetaNameValue) -> Result<Self, ParseAttributeOptionError> {
        if Self::left_hand_path_accepted(
            &name_value
                .path
                .get_ident()
                .ok_or(UnacceptableParseError::LeftHandSideValuePathIsNotIdent)?
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

    /// try parse the rule from a [`MetaList`]
    fn parse_meta_list(meta_list: &MetaList) -> Result<Self, ParseAttributeOptionError> {
        if Self::left_hand_path_accepted(
            &meta_list
                .path
                .get_ident()
                .ok_or(UnacceptableParseError::LeftHandSideValuePathIsNotIdent)?
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
/// that particular expression.
///
/// It is very specific but it is used to encapsulate code to parse option
#[must_use]
fn get_string_literal(expr: &Expr) -> Option<String> {
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

/// Auto implementation from [`FieldAttributeOptionParseUtils`] to an [`FieldAttributeOptionParse`]
impl<T: ParseOptionUtils> ParseOption for T {
    #[inline]
    fn parse_option(option: &Meta) -> Result<Self, ParseAttributeOptionError> {
        Self::parse_option_utils(option)
    }
}

// TODO review
/// Trait to convert an option to actual implementation code
pub trait ToCode {
    /// get the code with the [`FieldName`] information
    #[must_use]
    fn to_code(&self, field: &FieldInformation) -> TokenStream2;
}
