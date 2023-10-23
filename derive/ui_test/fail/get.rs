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

fn main() {}
