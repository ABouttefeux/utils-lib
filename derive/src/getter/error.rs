use std::{
    error::Error,
    fmt::{self, Display},
};

use super::option_enum::{ImmutableGetterAttributeOption, MutableGetterAttributeOption};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AttributeParseError {
    /// the attribute is a name value which is not supported yet
    NameValue,
    /// no attribute found
    NotFound,
    /// Parse error form syn
    ExprParseError(syn::Error),
}

impl From<syn::Error> for AttributeParseError {
    #[inline]
    fn from(value: syn::Error) -> Self {
        Self::ExprParseError(value)
    }
}

impl Display for AttributeParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::NameValue => write!(f, "field attribute is not supported in name value mode. Please refer to the documentation"),
            Self::NotFound => write!(f, "attribute #[get] or #[get_mut] not found"),
            Self::ExprParseError(err) => write!(f, "{err}"),
        }
    }
}

impl Error for AttributeParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NameValue | Self::NotFound => None,
            Self::ExprParseError(err) => Some(err),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum AcceptableParseError {
    /// There is no assignment and the path is not recognized for this option.
    ///
    /// Acceptable error.
    PathNotRecognized,
    /// Left hand side value in assignment is not recognized for this option.
    ///
    /// Acceptable error.
    LeftHandSideValueNotRecognized,
}

impl Display for AcceptableParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathNotRecognized => write!(
                f,
                "there is no assignment and the path is not recognized for this option"
            ),
            Self::LeftHandSideValueNotRecognized => write!(
                f,
                "left hand side value in assignment is not recognized for this option"
            ),
        }
    }
}

impl Error for AcceptableParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LeftHandSideValueNotRecognized | Self::PathNotRecognized => None,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum UnacceptableParseError {
    /// The left hand side path in an assignment has multiple section and is therefore not a ident
    LeftHandSideValuePathIsNotIdent,
    /// Right hand value in assignment is misformed or invalid
    RightHandValueInvalid,
    /// The right hand side value is not a literal string when it is expected
    RightHandNameValueExprNotLitString,
    /// Parse error form syn
    IdentParseError(syn::Error),
}

impl From<syn::Error> for UnacceptableParseError {
    #[inline]
    fn from(value: syn::Error) -> Self {
        Self::IdentParseError(value)
    }
}

impl Display for UnacceptableParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RightHandValueInvalid => {
                write!(f, "right hand value in assignment is misformed or invalid")
            }
            Self::IdentParseError(ref err) => write!(f, "syn ident parse error: {err}"),
            Self::LeftHandSideValuePathIsNotIdent => write!(f, "the left hand side path in an assignment has multiple section and is therefore not a ident"),
            Self::RightHandNameValueExprNotLitString => write!(f, "the right hand side value is not a literal string when it is expected"),
        }
    }
}

impl Error for UnacceptableParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RightHandValueInvalid
            | Self::RightHandNameValueExprNotLitString
            | Self::LeftHandSideValuePathIsNotIdent => None,
            Self::IdentParseError(ref err) => Some(err),
        }
    }
}

// TODO name
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AttributeOptionParseError {
    Acceptable(AcceptableParseError),
    Unacceptable(UnacceptableParseError),
}

impl From<AcceptableParseError> for AttributeOptionParseError {
    #[inline]
    fn from(value: AcceptableParseError) -> Self {
        Self::Acceptable(value)
    }
}

impl From<UnacceptableParseError> for AttributeOptionParseError {
    #[inline]
    fn from(value: UnacceptableParseError) -> Self {
        Self::Unacceptable(value)
    }
}

impl From<syn::Error> for AttributeOptionParseError {
    #[inline]
    fn from(value: syn::Error) -> Self {
        Self::from(UnacceptableParseError::from(value))
    }
}

impl Display for AttributeOptionParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acceptable(ref err) => write!(f, "{err}"),
            Self::Unacceptable(ref err) => write!(f, "{err}"),
        }
    }
}

impl Error for AttributeOptionParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Acceptable(ref err) => Some(err),
            Self::Unacceptable(ref err) => Some(err),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MutableAddConfigError {
    Acceptable(AcceptableParseError),
    Unacceptable(UnacceptableParseError, MutableGetterAttributeOption),
}

impl From<AcceptableParseError> for MutableAddConfigError {
    #[inline]
    fn from(value: AcceptableParseError) -> Self {
        Self::Acceptable(value)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ImmutableAddConfigError {
    Acceptable(AcceptableParseError),
    Unacceptable(UnacceptableParseError, ImmutableGetterAttributeOption),
}

impl From<MutableAddConfigError> for ImmutableAddConfigError {
    #[inline]
    fn from(value: MutableAddConfigError) -> Self {
        match value {
            MutableAddConfigError::Acceptable(err) => Self::Acceptable(err),
            MutableAddConfigError::Unacceptable(err, option) => {
                Self::Unacceptable(err, option.into())
            }
        }
    }
}

impl From<AcceptableParseError> for ImmutableAddConfigError {
    #[inline]
    fn from(value: AcceptableParseError) -> Self {
        Self::Acceptable(value)
    }
}
