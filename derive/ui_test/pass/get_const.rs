// pass test for const argument
use utils_lib_derive::Getter;

#[derive(Getter, Clone)]
struct S {
    #[get(constant)]
    f: usize,
    #[get(constant = "true")]
    f2: usize,
}

const C: S = S { f: 1, f2: 0 };

const fn cst_fn(s: &S) -> &usize {
    s.f()
}

const fn cst_fn_2(s: &S) -> &usize {
    s.f2()
}

fn main() {
    assert_eq!(cst_fn(&C), &1);
    assert_eq!(cst_fn_2(&C), &0);
}
