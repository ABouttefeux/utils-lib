mod attribute_option;
mod const_ty;
mod error;
mod getter_ty;
mod ident_option;
mod option;
mod option_enum;
mod self_ty;
mod visibility;
mod which_getter;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub use self::attribute_option::AttributeOptionParse;
use self::attribute_option::ToCode;
pub use self::error::AttributeParseError;
use self::option::{GetterOption, ImmutableGetterOption, MutableGetterOption};
use self::visibility::Visibility;

macro_rules! quote_compile_error {
    ($msg:expr $(,)?) => {
        quote! {compile_error!($msg)}.into()
    };
}

#[allow(clippy::module_name_repetitions)]
#[inline]
#[must_use]
pub fn derive_getter(item: TokenStream) -> TokenStream {
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

            iter.filter_map(|field| {
                let option = GetterOption::parse(&field.attrs);

                match option {
                    Ok(option) => Some(option.to_code(&field)),
                    Err(AttributeParseError::NotFound) => None,
                    Err(err) => {
                        println!("error parsing option: {err}");
                        None
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

    let name = input.ident;
    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let out = quote! {
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            #(#vec)*
        }
    }
    .into();

    println!("out:");
    println!("{out}");

    out
}
