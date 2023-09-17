//! Contain proc macro for `Getter` derive

mod attribute_option;
mod const_ty;
mod error;
mod field;
mod getter_ty;
mod ident_option;
mod option;
mod option_enum;
mod self_ty;
mod syntax;
mod visibility;
mod which_getter;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub use self::attribute_option::ParseOption;
use self::attribute_option::ToCode;
pub use self::error::AttributeParseError;
use self::field::Field;
use self::option::{GetterOption, ImmutableGetterOption, MutableGetterOption};
use self::visibility::Visibility;

/// Creates a quote with compile error with the given message
macro_rules! quote_compile_error {
    ($($tt:tt)* ) => {
        quote! {compile_error!($($tt)*);}.into()
    };
}

// TODO share option for both

/// Derive getter macro. see [`crate::derive_getter`]
#[inline]
#[must_use]
pub fn derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let vec: Vec<TokenStream2> = match input.data {
        Data::Struct(data) => {
            //
            let iter = match data.fields {
                Fields::Named(fields) => fields.named.into_iter(),
                Fields::Unnamed(fields) => fields.unnamed.into_iter(),
                Fields::Unit => {
                    // cspell: ignore fieldless
                    return quote_compile_error!(
                        "The trait getter cannot be derive on fieldless struct"
                    );
                }
            };

            iter.enumerate()
                .filter_map(|(field_index, field)| {
                    let option = GetterOption::parse(&field.attrs);

                    match option {
                        Ok(option) => Some(option.to_code(&Field::new(field, field_index))),
                        Err(AttributeParseError::NotFound) => None,
                        Err(err) => {
                            //println!("error parsing option: {err}");
                            let message = format!("error parsing option: {err}");
                            Some(quote_compile_error!(#message))
                        }
                    }
                })
                .collect::<Vec<TokenStream2>>()
        }
        Data::Enum(_) => {
            return quote_compile_error!("cannot derive getter for enums yet");
        }
        Data::Union(_) => {
            return quote_compile_error!("cannot derive getter for unions yet");
        }
    };

    let out = if vec.is_empty() {
        quote_compile_error!("no field has attribute #[get] or #[get_mut]")
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

    //println!("out:");
    //println!("{out}");

    out.into()
}
