use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get]
    f: usize,
}

fn main() {}
