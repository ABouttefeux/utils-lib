//! Contains [`ConstTy`]

use std::fmt::{self, Display};

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Expr, ExprLit, Lit, MetaNameValue};

use super::{
    attribute_option::{get_string_literal, ParseOptionUtils},
    error::{AcceptableParseError, ParseAttributeOptionError, UnacceptableParseError},
};

/// Option to determine if a getter should be constant or not.
/// By default the getter is not constant.
///
/// Accept value : like `#[get(const)]` or `#[get(const = true/false)]`.
/// - const (WIP) TODO
/// - Const
/// - constant
/// - Constant
/// - Const = true/false
/// - Const(true/false)
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

impl ParseOptionUtils for ConstTy {
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
    fn parse_name_value(name_value: &MetaNameValue) -> Result<Self, ParseAttributeOptionError> {
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
                Ok(lit_bool.value().into())
            } else {
                // this is the default behavior, see [`ParseOptionUtils::parse_name_value`]
                let string = get_string_literal(&name_value.value)
                    .ok_or(UnacceptableParseError::RightHandNameValueExprNotLitString)?;
                Self::parse_option_from_str_assignment(&string)
                    .ok_or_else(|| UnacceptableParseError::RightHandValueInvalid.into())
            }
        } else {
            Err(AcceptableParseError::LeftHandSideValueNotRecognized.into())
        }
    }

    #[inline]
    fn left_hand_path_accepted(path: &str) -> bool {
        path == "const" || path == "Const" || path == "constant" || path == "Constant"
    }
}

impl ToTokens for ConstTy {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(self.quote());
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

#[cfg(test)]
mod test {
    use super::ConstTy;

    #[test]
    fn const_ty() {
        assert_eq!(ConstTy::from(true), ConstTy::Constant);
        assert_eq!(ConstTy::from(false), ConstTy::NonConstant);

        assert!(bool::from(ConstTy::Constant));
        assert!(!bool::from(ConstTy::NonConstant));

        assert!(bool::from(ConstTy::from(true)));
        assert!(!bool::from(ConstTy::from(false)));

        assert_eq!(ConstTy::Constant.as_ref(), &true);
        assert_eq!(ConstTy::NonConstant.as_ref(), &false);
    }
}
