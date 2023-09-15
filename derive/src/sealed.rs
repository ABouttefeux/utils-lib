//! Contain proc macro for the `Sealed` trait derive and definition

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive the `Sealed` trait, see [`crate::derive_sealed`]
///
/// # Panic
///
/// panic if the derive macro is not applied to an struct, enum or union
#[inline]
#[must_use]
pub fn derive(item: TokenStream) -> TokenStream {
    // let item: TokenStream2 = item.into();
    // let name = find_name(&mut item.into_iter()).expect("no name found");

    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote!(
        #[automatically_derived]
        impl #impl_generics crate::private::Sealed for #name #ty_generics #where_clause {}
    )
    .into()
}

/// Creates a trait `Sealed` into a private module `private`.
#[inline]
#[must_use]
#[allow(clippy::module_name_repetitions)] // trait is not a valid function name
#[allow(clippy::needless_pass_by_value)] // the signature of a proc macro is to take by value
pub fn trait_sealed(item: TokenStream) -> TokenStream {
    if item.is_empty() {
        quote!(
            mod private {
                pub trait Sealed {}
            }
        )
    } else {
        quote!(compile_error!("trait_sealed!() does not take any arguments");)
    }
    .into()
}
