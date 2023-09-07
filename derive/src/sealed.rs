use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive the `Sealed` trait
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
