// Getter fail to be derived for enums
use utils_lib_derive::Getter;

#[derive(Getter)]
enum E {
    #[get]
    Variant,
}

fn main() {}
