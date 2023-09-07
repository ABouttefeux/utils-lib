use std::{
    error::Error,
    fmt::{self, Display},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
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
            Self::NameValue => write!(f, "Field attribute is not supported in name value mode. Please refer to the documentation"),
            Self::NotFound => write!(f, "Attribute #[get] or #[get_mut] not found"),
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
