// fail test for self type move but getter type by ref.
// It would create a dandling reference so it returns and custom error.
use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get(self_ty = "move", getter_ty = "by_ref")]
    f: usize,
}

fn main() {}
