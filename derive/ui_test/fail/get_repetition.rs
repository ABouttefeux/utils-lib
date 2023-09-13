use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get(public, public)]
    a: u32,
}

fn main() {}
