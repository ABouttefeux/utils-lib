// fail test for non const getter

use utils_lib_derive::Getter;

#[derive(Getter, Clone)]
struct S {
    #[get(Const = false)]
    f: usize,
}

const fn cst_fn(s: &S) -> &usize {
    s.f()
}

fn main() {
    let s = S { f: 1 };
    assert_eq!(cst_fn(&s), &1);
}
