use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{Expr, ExprLit, Lit, Meta, MetaList, MetaNameValue, Path};

// TODO code to avoid duplication for parsing option
// TODO think about error

/// trait for option element that are parsed from [`Meta`]
pub trait AttributeOptionParse {
    /// try to parse the option element from a [`Meta`] return [`Some`] if the element is valid
    /// [`None`] otherwise
    #[must_use]
    fn parse_option(option: &Meta) -> Option<Self>
    where
        Self: Sized;
}

/// trait for option element that are parsed from [`Meta`] providing default structure
/// to implement [`AttributeOptionParse`] more easily
pub trait AttributeOptionParseUtils: Sized {
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
    #[inline]
    #[must_use]
    fn parse_from_path(path: &Path) -> Option<Self> {
        path.get_ident()
            .and_then(|ident| Self::parse_from_ident(ident))
    }

    /// Try parse a Self from a [`Ident`] as the modifier
    #[inline]
    #[must_use]
    fn parse_from_ident(ident: &Ident) -> Option<Self> {
        Self::parse_option_from_str(&ident.to_string())
    }

    /// Try parse a Self from a [`Ident`] as an assignment
    #[inline]
    #[must_use]
    fn parse_from_ident_assignment(ident: &Ident) -> Option<Self> {
        Self::parse_option_from_str_assignment(&ident.to_string())
    }

    /// try to parse the option element from a [`Meta`] return [`Some`] if the element is valid
    /// [`None`] otherwise
    #[inline]
    #[must_use]
    fn parse_option_utils(option: &Meta) -> Option<Self> {
        match option {
            Meta::Path(path) => Self::parse_from_path(path),
            Meta::NameValue(name_value) => Self::parse_name_value(name_value),
            Meta::List(meta_list) => Self::parse_meta_list(meta_list),
        }
    }

    /// try parse the rule from a [`MetaNameValue`]
    #[inline]
    #[must_use]
    fn parse_name_value(name_value: &MetaNameValue) -> Option<Self> {
        if Self::left_hand_path_accepted(&name_value.path.get_ident()?.to_string()) {
            let string = get_string_literal(&name_value.value)?;
            Self::parse_option_from_str_assignment(&string)
        } else {
            None
        }
    }

    /// try parse the rule from a [`MetaList`]
    #[inline]
    #[must_use]
    fn parse_meta_list(meta_list: &MetaList) -> Option<Self> {
        if Self::left_hand_path_accepted(&meta_list.path.get_ident()?.to_string()) {
            Self::parse_from_ident_assignment(&meta_list.parse_args::<Ident>().ok()?)
        } else {
            None
        }
    }
}

/// Get the [`String`] value of a [`Lit::Str`] from [`Expr`] if it were
/// that particular expression.
///
/// It is very specific but it is used to encapsulate code to parse option
#[inline]
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

impl<T: AttributeOptionParseUtils> AttributeOptionParse for T {
    #[inline]
    fn parse_option(option: &Meta) -> Option<Self> {
        Self::parse_option_utils(option)
    }
}
