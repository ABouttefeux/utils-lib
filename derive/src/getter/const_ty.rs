//! Contains [`ConstTy`]

use std::fmt::{self, Display};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Expr, ExprLit, Field, Lit, MetaNameValue};

use super::{
    attribute_option::{FieldAttributeOptionParseUtils, ToCode},
    error::{AcceptableParseError, FieldAttributeOptionParseError, UnacceptableParseError},
};

/// Option to determine if a getter should be constant or not.
/// By default the getter is not constant.
///
/// Accept value : like `#[get(const)]` or `#[get(const = true/false)]`.
/// - const
/// - const = true/false
/// - const(true/false) //TODO
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
pub enum ConstTy {
    /// Non constant so the default `fn name()`.
    #[default]
    NonConstant = 0,
    /// Constant, i.e. `const fn name()`.
    Constant = 1,
}

impl ConstTy {
    /// return the token stream link to the const function part
    #[inline]
    pub fn quote(self) -> proc_macro2::TokenStream {
        match self {
            Self::Constant => quote! {const},
            Self::NonConstant => quote! {},
        }
    }
}

impl FieldAttributeOptionParseUtils for ConstTy {
    #[inline]
    fn parse_option_from_str(path: &str) -> Option<Self> {
        Self::left_hand_path_accepted(path).then_some(Self::Constant)
    }

    #[inline]
    fn parse_option_from_str_assignment(path: &str) -> Option<Self> {
        Self::parse_option_from_str(path).or_else(|| {
            if path == "true" {
                Some(Self::Constant)
            } else if path == "false" {
                Some(Self::NonConstant)
            } else {
                None
            }
        })
    }

    #[inline]
    fn parse_name_value(
        name_value: &MetaNameValue,
    ) -> Result<Self, FieldAttributeOptionParseError> {
        if Self::left_hand_path_accepted(
            &name_value
                .path
                .get_ident()
                .ok_or(UnacceptableParseError::LeftHandSideValuePathIsNotIdent)?
                .to_string(),
        ) {
            if let Expr::Lit(ExprLit {
                lit: Lit::Bool(lit_bool),
                ..
            }) = &name_value.value
            {
                if lit_bool.value() {
                    Ok(Self::Constant)
                } else {
                    Ok(Self::NonConstant)
                }
            } else {
                Err(UnacceptableParseError::RightHandValueInvalid.into())
            }
        } else {
            Err(AcceptableParseError::LeftHandSideValueNotRecognized.into())
        }
    }

    #[inline]
    fn left_hand_path_accepted(path: &str) -> bool {
        path == "const" || path == "constant" || path == "Constant"
    }
}

impl ToCode for ConstTy {
    #[inline]
    fn to_code(&self, _field: &Field) -> TokenStream2 {
        self.quote()
    }
}

impl Display for ConstTy {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constant => write!(f, "constant"),
            Self::NonConstant => write!(f, "non-constant"),
        }
    }
}

impl From<bool> for ConstTy {
    #[inline]
    fn from(value: bool) -> Self {
        if value {
            Self::Constant
        } else {
            Self::NonConstant
        }
    }
}

impl From<ConstTy> for bool {
    #[inline]
    fn from(value: ConstTy) -> Self {
        match value {
            ConstTy::Constant => true,
            ConstTy::NonConstant => false,
        }
    }
}

impl AsRef<bool> for ConstTy {
    #[inline]
    fn as_ref(&self) -> &bool {
        match self {
            Self::Constant => &true,
            Self::NonConstant => &false,
        }
    }
}
