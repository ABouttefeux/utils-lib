// fail test for non const getter
use utils_lib_derive::Getter;

#[derive(Getter, Clone)]
struct S {
    #[get(Const = false)]
    f: usize,
}

const fn cst_fn(s: &S) -> &usize {
    s.f() // f() is not const and therefore fail to compile
}

fn main() {}
