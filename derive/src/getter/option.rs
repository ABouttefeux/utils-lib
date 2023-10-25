//! Contain the option container [`GetterOption`] and [`super::which_getter::WhichGetter`]
//! variant [`MutableGetterOption`] and [`ImmutableGetterOption`]

#![allow(clippy::module_name_repetitions)] // TODO

use std::{collections::HashSet, hash::Hash};

use macro_utils::field::{Field, FieldInformation};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Meta, Path, Token};

use super::{
    attribute_option::ToCode,
    const_ty::ConstTy,
    error::{AddConfigError, GetterParseError, OptionValidationError, ParseAttributeOptionError},
    getter_ty::GetterTy,
    name::FunctionName,
    option_enum::{ImmutableOptionList, MutableOptionList, OptionList},
    self_ty::SelfTy,
    which_getter::WhichGetter,
    OptionParseError, ParseOption, Visibility,
};

/// the getter option
#[derive(Clone)]
pub struct GetterOption {
    /// The field information
    field: FieldInformation,
    /// the attribute option
    which: WhichGetter,
}

impl GetterOption {
    /// wrap the enum value
    #[inline]
    #[must_use]
    const fn new(field: FieldInformation, which: WhichGetter) -> Self {
        Self { field, which }
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
    #[must_use]
    fn is_valid_path_attribute(path: &Path) -> bool {
        Self::valid_attribute()
            .into_iter()
            .any(|s| path.is_ident(s))
    }

    // TODO
    // - if we want a mutable we write `#[get_mut]` with th same above rule or `#[get(mut)]`.
    // - if we want both mut and mut we write `#[get(add_mut)]` or `#[get_mut(add_imut)]`
    //  or `#[get(both)]`.

    /// - by default we would have `#[get]` it create a private getter.
    /// - if we want a public we have `#[get(pub)]`  or `#[get(visibility = pub)]`,
    /// possibilities are pub(...) public private.
    /// - if we want to rename we write `#[get(rename = "...")]`.
    pub fn parse(field: Field) -> Result<Self, OptionParseError> {
        /// merge a configuration with an option of a which getter
        #[must_use]
        fn add_option_config(out: Option<WhichGetter>, which: WhichGetter) -> WhichGetter {
            if let Some(s) = out {
                s.add_config(which)
            } else {
                which
            }
        }

        let mut out = None;

        for attribute in &field.field().attrs {
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
                        return Err(OptionParseError::NameValue);
                    }
                }
            }
        }

        let out = out.ok_or(OptionParseError::NotFound)?;

        let getter_option = Self::new(FieldInformation::from_field(field), out);
        getter_option.validate()?;
        Ok(getter_option)
    }

    // /// Merge two configuration giving the priority to the `other` config, see [`WhichGetter::add_config`]
    // fn add_config(self, other: WhichGetter) -> Self {
    //     Self::new(self.field, self.which.add_config(other))
    // }

    /// Verify that the option is valid
    fn validate(&self) -> Result<(), OptionValidationError> {
        match &self.which {
            WhichGetter::Immutable(immutable) => {
                if immutable
                    .option
                    .name()
                    .name(self.field.field_name())
                    .is_none()
                {
                    return Err(OptionValidationError::FunctionNameMissing);
                }
            }
            WhichGetter::Mutable(mutable) => {
                if mutable.name().name_mut(self.field.field_name()).is_none() {
                    return Err(OptionValidationError::FunctionNameMissing);
                }
            }
            WhichGetter::Both { immutable, mutable } => {
                if immutable
                    .option
                    .name()
                    .name(self.field.field_name())
                    .is_none()
                    || mutable.name().name_mut(self.field.field_name()).is_none()
                {
                    return Err(OptionValidationError::FunctionNameMissing);
                }
            }
        }

        self.which.validate()
    }
}

impl ToTokens for GetterOption {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(self.which.to_code(&self.field));
    }
}

//-------------------------

// TODO move
// TODO name

/// trait to avoid code repetition for [`ParseGetterOption::parse`] between
/// [`ImmutableGetterOption`] and [`MutableGetterOption`].
// the visibility is only require for the doc link in the doc of the error.
pub(super) trait ParseGetterOption: Sized + Default {
    /// The list of option, see [`OptionList`].
    type Option: OptionList + Hash + Eq;

    /// Try tp parse an iterator of [`Meta`] into a Option
    fn parse<T: IntoIterator<Item = Meta>>(
        tokens: T,
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

/// Option for immutable getter
#[derive(Clone, Default)]
pub struct ImmutableGetterOption {
    /// The base option that can be applied to a mutable ref getter
    option: MutableGetterOption,
    /// if the function is constant or not
    const_ty: ConstTy,
    /// if getter is by ref, value or the value is cloned
    ty: GetterTy,
    /// if the self value is borrowed or moved(or copied)
    self_ty: SelfTy,
}

impl ImmutableGetterOption {
    /// Verify that the option is valid
    pub fn validate(&self) -> Result<(), OptionValidationError> {
        self.option.validate()?;
        if self.self_ty == SelfTy::Value && self.ty == GetterTy::Ref {
            Err(OptionValidationError::SelfMoveOnReturnRef)
        } else {
            Ok(())
        }
    }
}

impl ParseGetterOption for ImmutableGetterOption {
    type Option = ImmutableOptionList;

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
            Err(ParseAttributeOptionError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
                    err,
                    ImmutableOptionList::ConstTy,
                ));
            }
            Err(ParseAttributeOptionError::Acceptable(_)) => {}
        }
        match GetterTy::parse_option(option) {
            Ok(ty) => {
                self.ty = ty;
                return Ok(ImmutableOptionList::GetterTy);
            }
            Err(ParseAttributeOptionError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
                    err,
                    ImmutableOptionList::GetterTy,
                ));
            }
            Err(ParseAttributeOptionError::Acceptable(_)) => {}
        }
        match SelfTy::parse_option(option) {
            Ok(self_ty) => {
                self.self_ty = self_ty;
                Ok(ImmutableOptionList::SelfTy)
            }
            Err(ParseAttributeOptionError::Unacceptable(err)) => Err(AddConfigError::Unacceptable(
                err,
                ImmutableOptionList::SelfTy,
            )),
            Err(ParseAttributeOptionError::Acceptable(err)) => Err(err.into()),
        }
    }
}

impl ToCode for ImmutableGetterOption {
    fn to_code(&self, field_information: &FieldInformation) -> TokenStream2 {
        let visibility = self.option.visibility();
        // TODO improve

        let fn_name = self
            .option
            .name()
            .name(field_information.field_name())
            .expect("no field name");
        let ty = field_information.ty();
        let field_name = field_information.field_name();

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
#[derive(Clone, Default)]
pub struct MutableGetterOption {
    /// visibility
    visibility: Visibility,
    /// name of the getter
    name: FunctionName,
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
    pub const fn name(&self) -> &FunctionName {
        &self.name
    }

    /// Verify that the option is valid
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    #[inline]
    pub const fn validate(&self) -> Result<(), OptionValidationError> {
        Ok(())
    }
}

impl ParseGetterOption for MutableGetterOption {
    type Option = MutableOptionList;

    /// try to add a option from a meta. Return true if it is a valid option, false otherwise.
    fn add_config(&mut self, option: &Meta) -> Result<Self::Option, AddConfigError<Self::Option>> {
        match Visibility::parse_option(option) {
            Ok(vis) => {
                self.visibility = vis;
                return Ok(MutableOptionList::Visibility);
            }
            Err(ParseAttributeOptionError::Unacceptable(err)) => {
                return Err(AddConfigError::Unacceptable(
                    err,
                    MutableOptionList::Visibility,
                ));
            }
            Err(ParseAttributeOptionError::Acceptable(_)) => {}
        }
        match FunctionName::parse_option(option) {
            Ok(name) => {
                self.name = name;
                Ok(MutableOptionList::IdentOption)
            }
            Err(ParseAttributeOptionError::Unacceptable(err)) => Err(AddConfigError::Unacceptable(
                err,
                MutableOptionList::IdentOption,
            )),
            Err(ParseAttributeOptionError::Acceptable(err)) => Err(err.into()),
        }
    }
}

impl ToCode for MutableGetterOption {
    fn to_code(&self, field_information: &FieldInformation) -> TokenStream2 {
        let visibility = self.visibility();
        // TODO improve
        let fn_name = self
            .name()
            .name_mut(field_information.field_name())
            .expect("no field name");
        let ty = &field_information.ty();
        let field_name = field_information.field_name();

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
