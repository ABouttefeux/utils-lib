#![allow(clippy::module_name_repetitions)] // TODO

use std::collections::HashSet;
use std::hash::Hash;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Field;
use syn::{punctuated::Punctuated, Attribute, Meta, Path, Token};

use super::attribute_option::ToCode;
use super::error::{AddConfigError, AttributeOptionParseError, GetterParseError};
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
                    if meta_list.path.is_ident(Self::IMMUTABLE) {
                        out = Some(add_option_config(
                            out,
                            WhichGetter::Immutable(ImmutableGetterOption::parse(list)?),
                        ));
                    } else if meta_list.path.is_ident(Self::MUTABLE) {
                        out = Some(add_option_config(
                            out,
                            WhichGetter::Mutable(MutableGetterOption::parse(list)?),
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

//TODO move

trait ParseGetterOption: Sized + Default {
    type Option: Hash;

    fn parse(
        tokens: impl IntoIterator<Item = Meta>,
    ) -> Result<Self, GetterParseError<ImmutableGetterAttributeOption>>; // TODO

    fn add_config(
        &mut self,
        option: &Meta,
    ) -> Result<ImmutableGetterAttributeOption, AddConfigError<ImmutableGetterAttributeOption>>;
}

// TODO trait

macro_rules! fn_parse_getter_option {
    ($tokens:ident) => {{
        let mut set = HashSet::new();
        let mut s = Self::default();
        for meta in $tokens {
            let res = s.add_config(&meta);
            match res {
                Ok(option) => {
                    if !set.insert(option) {
                        return Err(GetterParseError::AttributeOptionSetMultipleTimes(option));
                    }
                }
                Err(AddConfigError::Acceptable(_)) => { //continue;
                }
                Err(AddConfigError::Unacceptable(err, option)) => {
                    return Err(GetterParseError::AddConfigError(err, option))
                }
            }
        }
        Ok(s)
    }};
}

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

impl ImmutableGetterOption {
    #[inline]
    pub fn parse(
        tokens: impl IntoIterator<Item = Meta>,
    ) -> Result<Self, GetterParseError<ImmutableGetterAttributeOption>> {
        fn_parse_getter_option!(tokens)
    }

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    #[inline]
    fn add_config(
        &mut self,
        option: &Meta,
    ) -> Result<ImmutableGetterAttributeOption, AddConfigError<ImmutableGetterAttributeOption>>
    {
        match self.option.add_config(option) {
            Ok(option) => return Ok(option.into()),
            Err(err @ AddConfigError::Unacceptable(_, _)) => return Err(err.into()),
            Err(AddConfigError::Acceptable(_)) => {}
        }
        match ConstTy::parse_option(option) {
            Ok(const_ty) => {
                self.const_ty = const_ty;
                return Ok(ImmutableGetterAttributeOption::ConstTy);
            }
            Err(AttributeOptionParseError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
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
                return Err(AddConfigError::Unacceptable(
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
            Err(AttributeOptionParseError::Unacceptable(err)) => Err(AddConfigError::Unacceptable(
                err,
                ImmutableGetterAttributeOption::SelfTy,
            )),
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
        let const_ty = self.const_ty.to_code(field);
        let getter_ty_prefix = self.ty.prefix_quote();
        let getter_ty_suffix = self.ty.suffix_quote();
        let self_ty_code = self.self_ty.to_code(field);

        let comment = format!(
            "Getter on a {} of the field `{field_name}` with type [`{}`].",
            self.ty,
            ty.to_token_stream()
        );

        quote! {
            #[doc=#comment]
            #[inline]
            #[must_use]
            #visibility #const_ty fn #fn_name(#self_ty_code self) -> #getter_ty_prefix #ty {
                #getter_ty_prefix self.#field_name #getter_ty_suffix
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
    pub fn parse(
        tokens: impl IntoIterator<Item = Meta>,
    ) -> Result<Self, GetterParseError<MutableGetterAttributeOption>> {
        fn_parse_getter_option!(tokens)
    }

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    #[inline]
    fn add_config(
        &mut self,
        option: &Meta,
    ) -> Result<MutableGetterAttributeOption, AddConfigError<MutableGetterAttributeOption>> {
        match Visibility::parse_option(option) {
            Ok(vis) => {
                self.visibility = vis;
                return Ok(MutableGetterAttributeOption::Visibility);
            }
            Err(AttributeOptionParseError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
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
            Err(AttributeOptionParseError::Unacceptable(err)) => Err(AddConfigError::Unacceptable(
                err,
                MutableGetterAttributeOption::IdentOption,
            )),
            Err(AttributeOptionParseError::Acceptable(err)) => Err(err.into()),
        }
    }
}

impl ToCode for MutableGetterOption {
    #[inline]
    fn to_code(&self, field: &Field) -> TokenStream2 {
        let visibility = self.visibility().to_code(field);
        let fn_name = self.name().name_mut(field).expect("no field name");
        let ty = &field.ty;
        let field_name = field.ident.as_ref().expect("no field name");

        let comment = format!(
            "Getter on a mutable reference of the field {field_name} with type [`{}`].",
            ty.to_token_stream()
        );

        quote! {
            #[doc=#comment]
            #[inline]
            #[must_use]
            #visibility fn #fn_name(&mut self) -> &mut #ty {
                &mut self.#field_name
            }
        }
    }
}
