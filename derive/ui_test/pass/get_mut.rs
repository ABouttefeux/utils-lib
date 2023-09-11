use utils_lib_derive::Getter;

#[derive(Clone, Getter, Copy)]
struct S {
    #[get_mut]
    f: usize,
}

fn main() {
    let mut s = S { f: 0 };
    *s.f_mut() = 1;
}
