mod name {
    use self::m::S;

    fn t() {
        let s = S { f: 0 };
        assert_eq!(s.f(), &0);
    }

    pub mod m {
        use utils_lib_derive::Getter;

        #[derive(Getter)]
        pub struct S {
            #[get(visibility = "pub(super)")]
            pub f: usize,
        }
    }
}

use name::m::S;

fn main() {
    let s = S { f: 0 };
    assert_eq!(s.f(), &0);
}
