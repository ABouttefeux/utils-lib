// fail visibility test
mod name {
    use self::struct_def::S;

    fn ok() {
        let s = S { f: 0 };
        assert_eq!(s.f(), &0);
    }

    pub mod struct_def {
        use utils_lib_derive::Getter;

        #[derive(Getter)]
        pub struct S {
            // this should be visible in fn ok() but not in main()
            #[get(visibility = "pub(super)")]
            pub f: usize,
        }
    }
}

use name::struct_def::S;

fn main() {
    let s = S { f: 0 };
    assert_eq!(s.f(), &0);
}
