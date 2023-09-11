//! contains macro definitions

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
