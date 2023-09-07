use proc_macro2::Span;
use syn::{punctuated::Punctuated, Attribute, Ident, Meta, Path, Token};
use syn::{Expr, ExprLit, Lit};

use super::{
    const_ty::ConstTy, getter_ty::GetterTy, self_ty::SelfTy, which_getter::WhichGetter,
    AttributeOptionParse, AttributeParseError, Visibility,
};

/// [`WhichGetter`] wrapper
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GetterOption {
    /// wrapped value
    which: WhichGetter,
}

impl GetterOption {
    /// wrap the enum value
    #[inline]
    #[must_use]
    const fn new(which: WhichGetter) -> Self {
        Self { which }
    }

    /// Path string for immutable getter
    const IMMUTABLE: &'static str = "get";
    /// Path string for mutable reference getter
    const MUTABLE: &'static str = "get_mut";

    /// Get valid attribute path string
    #[inline]
    #[must_use]
    const fn valid_attribute() -> [&'static str; 2] {
        [Self::IMMUTABLE, Self::MUTABLE]
    }

    /// determine if the given path is a valid getter attribute
    #[inline]
    #[must_use]
    fn is_valid_path_attribute(path: &Path) -> bool {
        Self::valid_attribute()
            .into_iter()
            .any(|s| path.is_ident(s))
    }

    /// - by default we would have `#[get]` it create a private getter.
    /// - if we want a public we have ! #[get(pub)]!  or `#[get(visibility = pub)]`,
    /// possibilities are pub(...) public private.
    /// - if we want to rename we write `#[get(rename = "...")]`.
    /// - if we want a mutable we write `#[get_mut]` with th same above rule or `#[get(mut)]`.
    /// - if we want both mut and mut we write `#[get(add_mut)]` or `#[get_mut(add_imut)]`
    ///  or `#[get(both)]`.
    #[inline]
    pub fn parse(vec: &[Attribute]) -> Result<Self, AttributeParseError> {
        /// merge a configuration with an option of a which getter
        #[must_use]
        #[inline]
        fn add_option_config(out: Option<GetterOption>, which: WhichGetter) -> GetterOption {
            if let Some(s) = out {
                s.add_config(GetterOption::new(which))
            } else {
                GetterOption::new(which)
            }
        }

        let mut out = None;

        for attribute in vec {
            match &attribute.meta {
                Meta::List(meta_list) => {
                    let list = meta_list
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                    if meta_list.path.is_ident(Self::IMMUTABLE) {
                        out = Some(add_option_config(
                            out,
                            WhichGetter::Immutable(ImmutableGetterOption::parse(list)),
                        ));
                    } else if meta_list.path.is_ident(Self::MUTABLE) {
                        out = Some(add_option_config(
                            out,
                            WhichGetter::Mutable(MutableGetterOption::parse(list)),
                        ));
                    }
                }
                Meta::Path(path) => {
                    if path.is_ident(Self::IMMUTABLE) {
                        out = Some(add_option_config(
                            out,
                            WhichGetter::Immutable(ImmutableGetterOption::default()),
                        ));
                    } else if path.is_ident(Self::MUTABLE) {
                        out = Some(add_option_config(
                            out,
                            WhichGetter::Mutable(MutableGetterOption::default()),
                        ));
                    }
                }
                Meta::NameValue(name_value) => {
                    if Self::is_valid_path_attribute(&name_value.path) {
                        return Err(AttributeParseError::NameValue);
                    }
                }
            }
        }

        out.ok_or(AttributeParseError::NotFound)
    }

    /// Merge two configuration giving the priority to the `other` config, see [`WhichGetter::add_config`]
    fn add_config(self, other: Self) -> Self {
        Self::new(self.which.add_config(other.which))
    }
}

//-------------------------

// TODO validation
/// Option for immutable getter
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ImmutableGetterOption {
    /// The base option that can be applied to a mutable ref getter
    option: MutableGetterOption,
    const_ty: ConstTy,
    ty: GetterTy,
    self_ty: SelfTy,
}

/// macro for implementing the parse function for [`ImmutableGetterOption`] and [`MutableGetterOption`]
macro_rules! fn_parse_getter_option {
    ($tokens:ident) => {{
        // parse function (tokens: impl IntoIterator<Item = Meta>) -> Self
        let mut s = Self::default();
        for meta in $tokens {
            s.add_config(&meta);
        }
        s
    }};
}

impl ImmutableGetterOption {
    #[inline]
    #[must_use]
    pub fn parse(tokens: impl IntoIterator<Item = Meta>) -> Self {
        fn_parse_getter_option!(tokens)
    }

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    #[inline]
    fn add_config(&mut self, option: &Meta) -> bool {
        if self.option.add_config(option) {
            true
        } else if let Some(const_ty) = ConstTy::parse_option(option) {
            self.const_ty = const_ty;
            true
        } else if let Some(ty) = GetterTy::parse_option(option) {
            self.ty = ty;
            true
        } else if let Some(self_ty) = SelfTy::parse_option(option) {
            self.self_ty = self_ty;
            true
        } else {
            false
        }
    }
}

/// Option for mutable reference getter
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct MutableGetterOption {
    visibility: Visibility,
    name: Option<Ident>,
}

impl MutableGetterOption {
    #[inline]
    #[must_use]
    pub fn parse(tokens: impl IntoIterator<Item = Meta>) -> Self {
        fn_parse_getter_option!(tokens)
    }

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    #[inline]
    fn add_config(&mut self, option: &Meta) -> bool {
        if let Some(vis) = Visibility::parse_option(option) {
            self.visibility = vis;
            true
        } else if let Some(name) = Self::name_option(option) {
            self.name = Some(name);
            true
        } else {
            false
        }
    }

    /// Path string for the name option
    const NAME_PATH: &'static str = "name";

    /// Accepted values:
    /// - name = "..."
    /// - name(...) // TODO
    #[inline]
    #[must_use]
    fn name_option(option: &Meta) -> Option<Ident> {
        match option {
            Meta::NameValue(name_value) => {
                if name_value.path.is_ident(Self::NAME_PATH) {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(ref lit_string),
                        ..
                    }) = &name_value.value
                    {
                        return Some(Ident::new(&lit_string.value(), Span::call_site()));
                    }
                }
                None
            }
            Meta::List(meta_list) => todo!(),
            Meta::Path(_) => None,
        }
    }
}
