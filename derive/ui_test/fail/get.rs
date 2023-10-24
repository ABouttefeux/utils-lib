// fail test for missing field getter attribute or misformed attribute
use utils_lib_derive::Getter;

#[derive(Getter)]
struct Zst; // error on field less struct

#[derive(Getter)]
struct NoGet {} // error on field less struct

// error on unnamed getter
#[derive(Getter)]
struct Wrapper(
    #[get]
    #[get_mut]
    usize,
);

#[derive(Getter)]
struct S {
    #[get = "not valid"] // named value is not supported we are expecting #[get(...)]
    #[get_mut = "not valid"]
    f: usize,
}

// no #[get] or #[get_mut] found
#[derive(Getter)]
struct S2 {
    f: usize,
}

// test the case of multiple field attribute on the same field
// see ../pass/get.rs for the pass test
#[derive(Getter)]
struct MultipleAttribute {
    #[get(name = "field")] // this should be overridden
    #[get(name = "get")]
    f: (),
    #[get_mut(name = "field_mut")] // this should be overridden
    #[get_mut(name = "get_mut")]
    g: (),
}

fn main() {
    let mut m = MultipleAttribute { f: (), g: () };
    assert_eq!(m.field(), &());
    assert_eq!(m.field_mut(), &mut ());
}
