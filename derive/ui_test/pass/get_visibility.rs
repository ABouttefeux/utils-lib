use utils_lib_derive::Getter;

mod name {

    use super::*;

    #[derive(Getter, Clone, Copy)]
    pub struct S {
        #[get(public)]
        pub f: usize,
    }

    #[derive(Getter, Clone, Copy)]
    pub struct S2 {
        //#[get(pub(crate))]
        //#[get(visibility = "pub(super)")]
        #[get(public)]
        #[get_mut(public)]
        pub f: usize,
    }
}

use name::*;

fn main() {
    let s = S { f: 0 };
    assert_eq!(s.f(), &0);
    let mut s = S2 { f: 0 };
    assert_eq!(s.f(), &0);
    *s.f_mut() = 1;
    assert_eq!(s.f(), &1);
}
