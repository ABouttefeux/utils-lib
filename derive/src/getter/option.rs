#![allow(clippy::module_name_repetitions)] // TODO

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Field;
use syn::{punctuated::Punctuated, Attribute, Meta, Path, Token};

use super::attribute_option::ToCode;
use super::error::{AttributeOptionParseError, ImmutableAddConfigError, MutableAddConfigError};
use super::ident_option::IdentOption;
use super::option_enum::{ImmutableGetterAttributeOption, MutableGetterAttributeOption};
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
                    // FIXE ME
                    let list = meta_list
                        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                    println!("list parsed");
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

impl ToCode for GetterOption {
    #[inline]
    fn to_code(&self, field: &Field) -> TokenStream2 {
        self.which.to_code(field)
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
    fn add_config(
        &mut self,
        option: &Meta,
    ) -> Result<ImmutableGetterAttributeOption, ImmutableAddConfigError> {
        match self.option.add_config(option) {
            Ok(option) => return Ok(option.into()),
            Err(err @ MutableAddConfigError::Unacceptable(_, _)) => return Err(err.into()),
            Err(MutableAddConfigError::Acceptable(_)) => {}
        }
        match ConstTy::parse_option(option) {
            Ok(const_ty) => {
                self.const_ty = const_ty;
                return Ok(ImmutableGetterAttributeOption::ConstTy);
            }
            Err(AttributeOptionParseError::Unacceptable(err)) => {
                return Err(ImmutableAddConfigError::Unacceptable(
                    err,
                    ImmutableGetterAttributeOption::ConstTy,
                ));
            }
            Err(AttributeOptionParseError::Acceptable(_)) => {}
        }
        match GetterTy::parse_option(option) {
            Ok(ty) => {
                self.ty = ty;
                return Ok(ImmutableGetterAttributeOption::GetterTy);
            }
            Err(AttributeOptionParseError::Unacceptable(err)) => {
                return Err(ImmutableAddConfigError::Unacceptable(
                    err,
                    ImmutableGetterAttributeOption::GetterTy,
                ));
            }
            Err(AttributeOptionParseError::Acceptable(_)) => {}
        }
        match SelfTy::parse_option(option) {
            Ok(self_ty) => {
                self.self_ty = self_ty;
                Ok(ImmutableGetterAttributeOption::SelfTy)
            }
            Err(AttributeOptionParseError::Unacceptable(err)) => Err(
                ImmutableAddConfigError::Unacceptable(err, ImmutableGetterAttributeOption::SelfTy),
            ),
            Err(AttributeOptionParseError::Acceptable(err)) => Err(err.into()),
        }
    }
}

impl ToCode for ImmutableGetterOption {
    #[inline]
    fn to_code(&self, field: &Field) -> TokenStream2 {
        let visibility = self.option.visibility().to_code(field);
        let fn_name = self.option.name().name(field).expect("no field name");
        let ty = &field.ty;
        let field_name = field.ident.as_ref().expect("no field name");

        quote! {
            #[inline]
            #[must_use]
            #visibility fn #fn_name(&mut self) -> &#ty {
                &self.#field_name
            }
        }
    }
}

/// Option for mutable reference getter
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct MutableGetterOption {
    visibility: Visibility,
    name: IdentOption,
}

impl MutableGetterOption {
    #[inline]
    #[must_use]
    pub const fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    #[inline]
    #[must_use]
    pub const fn name(&self) -> &IdentOption {
        &self.name
    }

    #[inline]
    #[must_use]
    pub fn parse(tokens: impl IntoIterator<Item = Meta>) -> Self {
        fn_parse_getter_option!(tokens)
    }

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    #[inline]
    fn add_config(
        &mut self,
        option: &Meta,
    ) -> Result<MutableGetterAttributeOption, MutableAddConfigError> {
        match Visibility::parse_option(option) {
            Ok(vis) => {
                self.visibility = vis;
                return Ok(MutableGetterAttributeOption::Visibility);
            }
            Err(AttributeOptionParseError::Unacceptable(err)) => {
                return Err(MutableAddConfigError::Unacceptable(
                    err,
                    MutableGetterAttributeOption::Visibility,
                ));
            }
            Err(AttributeOptionParseError::Acceptable(_)) => {}
        }
        match IdentOption::parse_option(option) {
            Ok(name) => {
                self.name = name;
                Ok(MutableGetterAttributeOption::IdentOption)
            }
            Err(AttributeOptionParseError::Unacceptable(err)) => Err(
                MutableAddConfigError::Unacceptable(err, MutableGetterAttributeOption::IdentOption),
            ),
            Err(AttributeOptionParseError::Acceptable(err)) => Err(err.into()),
        }
    }

    // /// Accepted values:
    // /// - name = "..."
    // /// - name(...) // TODO
    // #[inline]
    // #[must_use]
    // fn name_option(option: &Meta) -> Option<Ident> {
    //     match option {
    //         Meta::NameValue(name_value) => {
    //             if name_value.path.is_ident(Self::NAME_PATH) {
    //                 if let Expr::Lit(ExprLit {
    //                     lit: Lit::Str(ref lit_string),
    //                     ..
    //                 }) = &name_value.value
    //                 {
    //                     return Some(Ident::new(&lit_string.value(), Span::call_site()));
    //                 }
    //             }
    //             None
    //         }
    //         Meta::List(meta_list) => todo!(),
    //         Meta::Path(_) => None,
    //     }
    // }
}

impl ToCode for MutableGetterOption {
    #[inline]
    fn to_code(&self, field: &Field) -> TokenStream2 {
        let visibility = self.visibility().to_code(field);
        let fn_name = self.name().name_mut(field).expect("no field name");
        let ty = &field.ty;
        let field_name = field.ident.as_ref().expect("no field name");

        quote! {
            #[inline]
            #[must_use]
            #visibility fn #fn_name(&mut self) -> &mut #ty {
                &mut self.#field_name
            }
        }
    }
}
