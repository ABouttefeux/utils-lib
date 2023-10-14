use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get(self_ty = "value", by_value)]
    a: u32,
}

#[derive(Getter)]
struct S2 {
    #[get(getter_ty = "by_value")]
    vec: Vec<()>,
}

#[derive(Getter, Clone)]
struct S3 {
    #[get(getter_ty = "by_value", self_ty = "value")]
    f3: String,
    #[get(getter_ty(Clone))]
    f4: String,
}

fn main() {
    let s = S { a: 0 };
    let _ = s.a();
    // s is moved and no longer valid
    let _ = s.a();

    let s2 = S3 {
        f3: "s3".to_owned(),
        f4: "s4".to_owned(),
    };

    assert_eq!(s2.f3(), "s3".to_owned()); // we "forgot" to clone s which lead to an error
    assert_eq!(s2.f4(), "s4".to_owned());
}
