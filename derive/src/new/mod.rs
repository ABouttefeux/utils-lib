//! Contain proc macro for `New` derive

mod attribute;
mod field;
mod option;
mod option_enum;
mod option_struct;

use macro_utils::quote_compile_error;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

// see [`crate::derive_new`]
#[must_use]
pub fn derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match input.data {
        Data::Struct(data) => {}
        Data::Enum(data) => {
            return quote_compile_error!("It is not possible to derive new for enum yet.");
        }
        Data::Union(data) => {
            return quote_compile_error!("It is not possible to derive new for unions.");
        }
    }

    todo!()
}
