use utils_lib_derive::Getter;

#[derive(Getter, Clone)]
struct S {
    #[get(constant)]
    f: usize,
}

const C: S = S { f: 1 };

const fn cst_fn() -> &'static usize {
    C.f()
}

fn main() {
    assert_eq!(cst_fn(), &1);
}