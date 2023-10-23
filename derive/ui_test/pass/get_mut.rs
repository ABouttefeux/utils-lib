// pass test for mut getter
use utils_lib_derive::Getter;

#[derive(Getter, Clone, Copy)]
struct S {
    #[get_mut]
    f: usize,
}

fn main() {
    let mut s = S { f: 0 };
    assert_eq!(s.f_mut(), &0);
    *s.f_mut() = 1;
    assert_eq!(s.f, 1);
}
