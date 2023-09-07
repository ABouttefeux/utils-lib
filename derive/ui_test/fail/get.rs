use utils_lib_derive::Getter;

#[derive(Getter)]
struct ZST;

#[derive(Getter)]
struct wrapper(#[get] usize);

fn main() {
    let w = Wrapper(0);
}
