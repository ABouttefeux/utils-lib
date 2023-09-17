#![allow(clippy::module_name_repetitions)] // TODO

use std::collections::HashSet;
use std::hash::Hash;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Index;
use syn::{punctuated::Punctuated, Attribute, Meta, Path, Token};

use super::attribute_option::ToCode;
use super::error::{AddConfigError, GetterParseError, ParseOptionError};
use super::field::Field;
use super::ident_option::IdentOption;
use super::option_enum::{ImmutableOptionList, MutableOptionList, OptionList};
use super::{
    const_ty::ConstTy, getter_ty::GetterTy, self_ty::SelfTy, which_getter::WhichGetter,
    AttributeParseError, ParseOption, Visibility,
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

// TODO move
// TODO name
/// trait to avoid code repetition for [`ParseGetterOption::parse`] between
/// [`ImmutableGetterOption`] and [`MutableGetterOption`].
trait ParseGetterOption: Sized + Default {
    /// The list of option, see [`OptionList`].
    type Option: OptionList + Hash + Eq;

    /// Try tp parse an iterator of [`Meta`] into a Option
    #[inline]
    fn parse(
        tokens: impl IntoIterator<Item = Meta>,
    ) -> Result<Self, GetterParseError<Self::Option>> {
        let mut set = HashSet::new();
        let mut s = Self::default();
        for meta in tokens {
            let res = s.add_config(&meta);
            match res {
                Ok(option) => {
                    // this replace function save us to do one clone
                    // as we get back the option
                    if let Some(option) = set.replace(option) {
                        return Err(GetterParseError::FieldAttributeOptionSetMultipleTimes(
                            option,
                        ));
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
    }

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    fn add_config(&mut self, option: &Meta) -> Result<Self::Option, AddConfigError<Self::Option>>;
}

// TODO validation
/// Option for immutable getter
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ImmutableGetterOption {
    /// The base option that can be applied to a mutable ref getter
    option: MutableGetterOption,
    /// if the funcion is constant or not
    const_ty: ConstTy,
    /// if getter is by ref, value or the value is cloned
    ty: GetterTy,
    /// if the self value is borrowed or moved(or copied)
    self_ty: SelfTy,
}

impl ParseGetterOption for ImmutableGetterOption {
    type Option = ImmutableOptionList;

    #[inline]
    fn add_config(&mut self, option: &Meta) -> Result<Self::Option, AddConfigError<Self::Option>> {
        match self.option.add_config(option) {
            Ok(option) => return Ok(option.into()),
            Err(err @ AddConfigError::Unacceptable(_, _)) => return Err(err.into()),
            Err(AddConfigError::Acceptable(_)) => {}
        }
        match ConstTy::parse_option(option) {
            Ok(const_ty) => {
                self.const_ty = const_ty;
                return Ok(ImmutableOptionList::ConstTy);
            }
            Err(ParseOptionError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
                    err,
                    ImmutableOptionList::ConstTy,
                ));
            }
            Err(ParseOptionError::Acceptable(_)) => {}
        }
        match GetterTy::parse_option(option) {
            Ok(ty) => {
                self.ty = ty;
                return Ok(ImmutableOptionList::GetterTy);
            }
            Err(ParseOptionError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
                    err,
                    ImmutableOptionList::GetterTy,
                ));
            }
            Err(ParseOptionError::Acceptable(_)) => {}
        }
        match SelfTy::parse_option(option) {
            Ok(self_ty) => {
                self.self_ty = self_ty;
                Ok(ImmutableOptionList::SelfTy)
            }
            Err(ParseOptionError::Unacceptable(err)) => Err(AddConfigError::Unacceptable(
                err,
                ImmutableOptionList::SelfTy,
            )),
            Err(ParseOptionError::Acceptable(err)) => Err(err.into()),
        }
    }
}

impl ToCode for ImmutableGetterOption {
    #[inline]
    fn to_code(&self, field: &Field) -> TokenStream2 {
        let visibility = self.option.visibility();
        // TODO improve
        let fn_name = self
            .option
            .name()
            .name(field.field())
            .expect("no field name");
        let ty = &field.field().ty;
        let field_name = field.field().ident.as_ref().map_or_else(
            || Index::from(field.index()).into_token_stream(),
            ToTokens::to_token_stream,
        );

        let const_ty = self.const_ty;
        let getter_ty_prefix = self.ty.prefix_quote();
        let getter_ty_suffix = self.ty.suffix_quote();
        let self_ty_code = self.self_ty;

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
    /// visibility
    visibility: Visibility,
    /// name of the getter
    name: IdentOption,
}

impl MutableGetterOption {
    /// getter on the visibility
    #[inline]
    #[must_use]
    pub const fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    /// getter on the name
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &IdentOption {
        &self.name
    }
}

impl ParseGetterOption for MutableGetterOption {
    type Option = MutableOptionList;

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    #[inline]
    fn add_config(&mut self, option: &Meta) -> Result<Self::Option, AddConfigError<Self::Option>> {
        match Visibility::parse_option(option) {
            Ok(vis) => {
                self.visibility = vis;
                return Ok(MutableOptionList::Visibility);
            }
            Err(ParseOptionError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
                    err,
                    MutableOptionList::Visibility,
                ));
            }
            Err(ParseOptionError::Acceptable(_)) => {}
        }
        match IdentOption::parse_option(option) {
            Ok(name) => {
                self.name = name;
                Ok(MutableOptionList::IdentOption)
            }
            Err(ParseOptionError::Unacceptable(err)) => Err(AddConfigError::Unacceptable(
                err,
                MutableOptionList::IdentOption,
            )),
            Err(ParseOptionError::Acceptable(err)) => Err(err.into()),
        }
    }
}

impl ToCode for MutableGetterOption {
    #[inline]
    fn to_code(&self, field: &Field) -> TokenStream2 {
        let visibility = self.visibility();
        // TODO improve
        let fn_name = self.name().name_mut(field.field()).expect("no field name");
        let ty = &field.field().ty;
        let field_name = field.field().ident.as_ref().map_or_else(
            || Index::from(field.index()).into_token_stream(),
            ToTokens::to_token_stream,
        );

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
