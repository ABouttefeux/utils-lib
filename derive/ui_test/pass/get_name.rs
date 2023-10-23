// pass test for name argument
use utils_lib_derive::Getter;

#[derive(Getter)]
struct Wrapper(#[get(name = "f")] usize);

#[derive(Getter)]
struct S {
    #[get(name = "field")]
    #[get_mut(name(field_mut))]
    f: usize,
}

fn main() {
    let w = Wrapper(0);
    assert_eq!(w.f(), &0);
    let mut s = S { f: 0 };
    assert_eq!(s.field(), &0);
    assert_eq!(s.field_mut(), &mut 0);
}
