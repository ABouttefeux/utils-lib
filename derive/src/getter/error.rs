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
pub enum OptionParseError {
    /// the attribute is a name value which is not supported yet
    NameValue,
    /// no attribute found
    NotFound,
    /// Parse error form syn
    ExprParseError(syn::Error),
    /// Error while parsing option for getter in the filed attribute
    // TODO Name
    GetterParseError(GetterParseError<ImmutableOptionList>),
    /// Error during the validation of the option, see [`OptionValidationError`]
    OptionValidationError(OptionValidationError),
}

impl From<OptionValidationError> for OptionParseError {
    #[inline]
    fn from(value: OptionValidationError) -> Self {
        Self::OptionValidationError(value)
    }
}

impl From<syn::Error> for OptionParseError {
    #[inline]
    fn from(value: syn::Error) -> Self {
        Self::ExprParseError(value)
    }
}

impl<T> From<T> for OptionParseError
where
    T: Into<GetterParseError<ImmutableOptionList>>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self::GetterParseError(value.into())
    }
}

impl Display for OptionParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::NameValue => write!(f, "field attribute is not supported in name value mode, please refer to the documentation"),
            Self::NotFound => write!(f, "attribute #[get] or #[get_mut] not found"),
            Self::ExprParseError(ref err) => write!(f, "{err}"),
            Self::GetterParseError(ref err) => write!(f, "{err}"),
            Self::OptionValidationError(ref err) => write!(f, "{err}"),
        }
    }
}

impl Error for OptionParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NameValue | Self::NotFound => None,
            Self::ExprParseError(ref err) => Some(err),
            Self::GetterParseError(ref err) => Some(err),
            Self::OptionValidationError(ref err) => Some(err),
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
pub enum ParseAttributeOptionError {
    /// Recoverable error that just signal that the option wasn't found by this attribute,
    /// see [`AcceptableParseError`].
    Acceptable(AcceptableParseError),
    /// Unrecoverable error that should lead to a compile error. This usually means an
    /// error in the parsing, see [`UnacceptableParseError`].
    Unacceptable(UnacceptableParseError),
}

impl From<AcceptableParseError> for ParseAttributeOptionError {
    #[inline]
    fn from(value: AcceptableParseError) -> Self {
        Self::Acceptable(value)
    }
}

impl From<UnacceptableParseError> for ParseAttributeOptionError {
    #[inline]
    fn from(value: UnacceptableParseError) -> Self {
        Self::Unacceptable(value)
    }
}

impl From<syn::Error> for ParseAttributeOptionError {
    #[inline]
    fn from(value: syn::Error) -> Self {
        Self::from(UnacceptableParseError::from(value))
    }
}

impl Display for ParseAttributeOptionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acceptable(ref err) => write!(f, "{err}"),
            Self::Unacceptable(ref err) => write!(f, "{err}"),
        }
    }
}

impl Error for ParseAttributeOptionError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Acceptable(ref err) => Some(err),
            Self::Unacceptable(ref err) => Some(err),
        }
    }
}

/// Error return by [`super::option::ParseGetterOption::add_config`]. This represent an
/// error while trying to add an new option to the configuration. The attribute could represent
/// no option and return [`Self::Acceptable`] and be skipped. Or return an error for a certain
/// option represented by `T` (of trait [`OptionList`]) by te variant [`Self::Unacceptable`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AddConfigError<T: OptionList> {
    /// Recoverable error. This means that no option for this attribute was found
    /// and therefore it was skipped
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

/// Error return by [`super::option::ParseGetterOption::parse`]. It is the error returned by
/// parsing a [`supper::which_getter::WhichGetter`] variant, a getter (attribute) option.
/// It has either an unacceptable error from a [`AddConfigError::Unacceptable`],
/// [`Self::AddConfigError`] or an error from adding the same option multiple time
/// [`Self::FieldAttributeOptionSetMultipleTimes`].
///
/// Note here that we stop propagating the [`AddConfigError::Acceptable`] variant
/// because as we said it was just a way to signal that any option wasn't found
/// and shouldn't lead to an compile error. Maybe latter I will convert that to an error.
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

/// Error return by validation function that verify the integrity of the configuration.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum OptionValidationError {
    /// name = \"#\" is missing and there is no default name for tuple struct
    FunctionNameMissing,
    /// self_ty is value but getter_ty is reference which is not valid,
    /// it create a dandling reference which the borrow checker reject
    SelfMoveOnReturnRef,
}

impl Display for OptionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunctionNameMissing => write!(
                f,
                "name = \"#\" is missing and there is no default name for tuple struct"
            ),
            Self::SelfMoveOnReturnRef => write!(
                f,
                "self_ty is value but getter_ty is reference which is not valid, \
                it create a dandling reference which the borrow checker reject"
            ),
        }
    }
}

impl Error for OptionValidationError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FunctionNameMissing | Self::SelfMoveOnReturnRef => None,
        }
    }
}
