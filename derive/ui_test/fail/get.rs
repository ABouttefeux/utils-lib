use utils_lib_derive::Getter;

#[derive(Getter)]
struct Zst;

#[derive(Getter)]
struct Wrapper(
    #[get]
    #[get_mut]
    usize,
);

#[derive(Getter)]
struct S {
    #[get = "not valid"]
    #[get_mut = "not valid"]
    f: usize,
}

#[derive(Getter)]
struct S2 {
    f: usize,
}

fn main() {}
