//! Contain proc macro for `Getter` derive

mod attribute_option;
mod const_ty;
mod error;
mod getter_ty;
mod name;
mod option;
mod option_enum;
mod self_ty;
mod syntax;
mod visibility;
mod which_getter;

use macro_utils::field::Field;
use macro_utils::quote_compile_error;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub use self::attribute_option::ParseOption;
pub use self::error::OptionParseError;
use self::option::{GetterOption, ImmutableGetterOption, MutableGetterOption};
use self::visibility::Visibility;

// TODO share option for both
// TODO multiple error reporting on #[get] #[get_mut]
// TODO vec so more than one #[get] and #[get_mut] can be added

/// Derive getter macro. see [`crate::derive_getter`]
#[inline]
#[must_use]
pub fn derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let vec: Vec<TokenStream2> = match input.data {
        Data::Struct(data) => {
            let iter = match data.fields {
                Fields::Named(fields) => fields.named.into_iter(),
                Fields::Unnamed(fields) => fields.unnamed.into_iter(),
                Fields::Unit => {
                    // cspell: ignore fieldless
                    return quote_compile_error!(
                        "The trait getter cannot be derive on fieldless struct."
                    );
                }
            };

            iter.enumerate()
                .filter_map(|(field_index, field)| {
                    let field = Field::new(field, field_index);
                    let option = GetterOption::parse(field);

                    match option {
                        Ok(option) => Some(option.into_token_stream()),
                        Err(OptionParseError::NotFound) => None,
                        Err(err) => {
                            let message = format!("error parsing option: {err}");
                            Some(quote_compile_error!(#message))
                        }
                    }
                })
                .collect::<Vec<TokenStream2>>()
        }
        Data::Enum(_) => {
            return quote_compile_error!("It is not possible to derive getter for enums yet.");
        }
        Data::Union(_) => {
            return quote_compile_error!("It is not possible to derive getter for unions yet.");
        }
    };

    let out = if vec.is_empty() {
        let message = OptionParseError::NotFound.to_string();
        //"No field has attribute #[get] or #[get_mut] has been found."
        quote_compile_error!(#message)
    } else {
        let name = input.ident;
        let generics = input.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            /// Automatically generated implementation for getters
            #[automatically_derived]
            impl #impl_generics #name #ty_generics #where_clause {
                #(#vec)*
            }
        }
    };

    out.into()
}
