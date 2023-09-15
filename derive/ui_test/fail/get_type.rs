use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get(self_ty = "value", by_value)]
    a: u32,
}

fn main() {
    let s = S { a: 0 };
    let _ = s.a();
    // s is moved and no longer valid
    let _ = s.a();
}
