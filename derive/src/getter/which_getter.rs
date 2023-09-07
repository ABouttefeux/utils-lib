use super::{ImmutableGetterOption, MutableGetterOption};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WhichGetter {
    Immutable(ImmutableGetterOption),
    Mutable(MutableGetterOption),
    Both {
        immutable: ImmutableGetterOption,
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
}

impl Default for WhichGetter {
    #[inline]
    fn default() -> Self {
        Self::Immutable(ImmutableGetterOption::default())
    }
}