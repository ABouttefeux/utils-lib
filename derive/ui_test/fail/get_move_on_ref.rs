use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get(self_ty = "move")]
    f: usize,
}

fn main() {}
