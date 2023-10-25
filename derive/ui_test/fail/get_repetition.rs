// fail test for option that are set twice ore more
use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get(public, public)]
    #[get_mut(public, public)]
    a: u32,
}

fn main() {}
