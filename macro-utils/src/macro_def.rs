//! contains macro definitions

/// create a getter function
#[macro_export]
macro_rules! getter {
    ($(#[$meta:meta])* $v:vis fn $i:ident() -> $t:ty ) => {
        $(#[$meta])*
        $v fn $i(&self) -> &$t {
            &self.$i
        }
    };
    ($(#[$meta:meta])* $v:vis const fn $i:ident() -> $t:ty ) => {
        $(#[$meta])*
        $v const fn $i(&self) -> &$t {
            &self.$i
        }
    };
}

/// Creates a quote with compile error with the given message
#[macro_export]
macro_rules! quote_compile_error {
    ($($tt:tt)* ) => {
        quote::quote! {compile_error!($($tt)*);}.into()
    };
}
