use utils_lib_derive::Getter;

#[derive(Getter)]
struct Zst;

#[derive(Getter)]
struct Wrapper(#[get] usize);

fn main() {
    let w = Wrapper(0);
}
