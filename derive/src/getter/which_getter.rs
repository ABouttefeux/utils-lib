//! Contains [`WhichGetter`]

use macro_utils::field::FieldInformation;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::{
    attribute_option::ToCode, error::OptionValidationError, ImmutableGetterOption,
    MutableGetterOption,
};

/// Determine which getter type is being implemented.
#[derive(Clone)]
pub enum WhichGetter {
    /// Immutable getter.
    Immutable(ImmutableGetterOption),
    /// Mutable getter.
    Mutable(MutableGetterOption),
    /// Both the mutable getter and immutable getter.
    Both {
        /// immut getter
        immutable: ImmutableGetterOption,
        /// mut getter
        mutable: MutableGetterOption,
    },
}

impl WhichGetter {
    /// Merge two config with other being the one being prioritized
    #[inline]
    pub fn add_config(self, other: Self) -> Self {
        #[allow(clippy::match_same_arms)] // readability (it is already not great)
        match (self, other) {
            // other is Self::Mutable
            (Self::Mutable(_), Self::Mutable(m)) => Self::Mutable(m),
            (
                Self::Immutable(i)
                | Self::Both {
                    immutable: i,
                    mutable: _,
                },
                Self::Mutable(m),
            ) => Self::Both {
                immutable: i,
                mutable: m,
            },
            // other is Self::Immutable
            (
                Self::Mutable(m)
                | Self::Both {
                    immutable: _,
                    mutable: m,
                },
                Self::Immutable(i),
            ) => Self::Both {
                immutable: i,
                mutable: m,
            },
            (Self::Immutable(_), Self::Immutable(i)) => Self::Immutable(i),
            // other is Self::Both
            (_, output @ Self::Both { .. }) => output,
        }
    }

    /// Verify that the option is valid
    pub fn validate(&self) -> Result<(), OptionValidationError> {
        match self {
            Self::Immutable(immutable) => immutable.validate(),
            Self::Mutable(mutable) => mutable.validate(),
            Self::Both { immutable, mutable } => {
                mutable.validate()?;
                immutable.validate()
            }
        }
    }
}

impl ToCode for WhichGetter {
    #[inline]
    fn to_code(&self, field: &FieldInformation) -> TokenStream2 {
        match self {
            Self::Immutable(i) => i.to_code(field),
            Self::Mutable(m) => m.to_code(field),
            Self::Both { immutable, mutable } => {
                let i_code = immutable.to_code(field);
                let m_code = mutable.to_code(field);
                quote! {
                    #i_code

                    #m_code
                }
            }
        }
    }
}

impl Default for WhichGetter {
    #[inline]
    fn default() -> Self {
        Self::Immutable(ImmutableGetterOption::default())
    }
}
