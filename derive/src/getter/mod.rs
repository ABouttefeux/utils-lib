mod attribute_option;
mod const_ty;
mod error;
mod getter_ty;
mod option;
mod self_ty;
mod visibility;
mod which_getter;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub use self::attribute_option::AttributeOptionParse;
pub use self::error::AttributeParseError;
use self::option::{GetterOption, ImmutableGetterOption, MutableGetterOption};
use self::visibility::Visibility;

#[inline]
#[must_use]
pub fn derive_getter(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    //let vec = Vec::new();

    match input.data {
        Data::Struct(data) => {
            //
            let iter = match data.fields {
                Fields::Named(fields) => fields.named.into_iter(),
                Fields::Unnamed(fields) => fields.unnamed.into_iter(),
                Fields::Unit => {
                    // cspell: ignore fieldless
                    panic!("The trait getter cannot be derive on fieldless struct");
                }
            };

            for field in iter {
                let option = GetterOption::parse(&field.attrs);
            }
        }
        Data::Enum(data) => {
            panic!("cannot derive getter for enums yet");
        }
        Data::Union(data) => {
            panic!("cannot derive getter for unions yet");
        }
    }

    let name = input.ident;
    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            //#(#vec)*
        }
    }
    .into()
}
