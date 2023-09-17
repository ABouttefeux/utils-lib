//! Contains the different error definitions

use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

use super::option_enum::{ImmutableOptionList, MutableOptionList, OptionList};

// TODO names

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
    /// Error while parsing option for getter in the filed attribute
    // TODO Name
    GetterParseError(GetterParseError<ImmutableOptionList>),
}

impl From<syn::Error> for AttributeParseError {
    #[inline]
    fn from(value: syn::Error) -> Self {
        Self::ExprParseError(value)
    }
}

impl<T> From<T> for AttributeParseError
where
    T: Into<GetterParseError<ImmutableOptionList>>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self::GetterParseError(value.into())
    }
}

impl Display for AttributeParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::NameValue => write!(f, "field attribute is not supported in name value mode, please refer to the documentation"),
            Self::NotFound => write!(f, "attribute #[get] or #[get_mut] not found"),
            Self::ExprParseError(ref err) => write!(f, "{err}"),
            Self::GetterParseError(ref err) => write!(f, "{err}")
        }
    }
}

impl Error for AttributeParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NameValue | Self::NotFound => None,
            Self::ExprParseError(ref err) => Some(err),
            Self::GetterParseError(ref err) => Some(err),
        }
    }
}

/// Parse error that should not cause compile error. It is just way of reporting
/// that the parsed stream is not describing a given option. But that we should
/// try for another option.
///
/// It is a recoverable error
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

/// Unrecoverable error that should be reported in a compile error.
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

/// Error given while trying to parse a option of a field attribute.
/// It could be that it is not applicable for the option and give [`Self::Acceptable`].
/// Or [`Self::Unacceptable`] means that the error is not recoverable and
/// should lead to a compile error.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ParseOptionError {
    Acceptable(AcceptableParseError),
    Unacceptable(UnacceptableParseError),
}

impl From<AcceptableParseError> for ParseOptionError {
    #[inline]
    fn from(value: AcceptableParseError) -> Self {
        Self::Acceptable(value)
    }
}

impl From<UnacceptableParseError> for ParseOptionError {
    #[inline]
    fn from(value: UnacceptableParseError) -> Self {
        Self::Unacceptable(value)
    }
}

impl From<syn::Error> for ParseOptionError {
    #[inline]
    fn from(value: syn::Error) -> Self {
        Self::from(UnacceptableParseError::from(value))
    }
}

impl Display for ParseOptionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acceptable(ref err) => write!(f, "{err}"),
            Self::Unacceptable(ref err) => write!(f, "{err}"),
        }
    }
}

impl Error for ParseOptionError {
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
pub enum AddConfigError<T: OptionList> {
    /// No configuration could added for Acceptable reason.
    Acceptable(AcceptableParseError),
    /// Error while trying to add given configuration.
    Unacceptable(UnacceptableParseError, T),
}

impl From<AddConfigError<MutableOptionList>> for AddConfigError<ImmutableOptionList> {
    #[inline]
    fn from(value: AddConfigError<MutableOptionList>) -> Self {
        match value {
            AddConfigError::Acceptable(err) => Self::Acceptable(err),
            AddConfigError::Unacceptable(err, option) => Self::Unacceptable(err, option.into()),
        }
    }
}

impl<T: OptionList> From<AcceptableParseError> for AddConfigError<T> {
    #[inline]
    fn from(value: AcceptableParseError) -> Self {
        Self::Acceptable(value)
    }
}

impl<T: OptionList + Display> Display for AddConfigError<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acceptable(ref err) => write!(f, "{err}"),
            Self::Unacceptable(ref err, ref option) => {
                write!(f, "got error {err} while parsing option {option}")
            }
        }
    }
}

impl<T: OptionList + Debug + Display> Error for AddConfigError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Acceptable(ref err) => Some(err),
            Self::Unacceptable(ref err, _) => Some(err),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum GetterParseError<T: OptionList> {
    /// Error while trying to add given configuration.
    AddConfigError(UnacceptableParseError, T),
    /// This attribute option is set multiple time we only accept it once.
    FieldAttributeOptionSetMultipleTimes(T),
}

impl<T: OptionList + Display> Display for GetterParseError<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldAttributeOptionSetMultipleTimes(ref option) => {
                write!(f, "{option} is set multiple times")
            }
            Self::AddConfigError(ref err, ref option) => {
                write!(f, "got error {err} while parsing option {option}")
            }
        }
    }
}

impl<T: OptionList + Display + Debug> Error for GetterParseError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FieldAttributeOptionSetMultipleTimes(_) => None,
            Self::AddConfigError(ref err, _) => Some(err),
        }
    }
}

impl From<GetterParseError<MutableOptionList>> for GetterParseError<ImmutableOptionList> {
    #[inline]
    fn from(value: GetterParseError<MutableOptionList>) -> Self {
        match value {
            GetterParseError::FieldAttributeOptionSetMultipleTimes(opt) => {
                Self::FieldAttributeOptionSetMultipleTimes(opt.into())
            }
            GetterParseError::AddConfigError(err, option) => {
                Self::AddConfigError(err, option.into())
            }
        }
    }
}
