// general getter derive pass test
use utils_lib_derive::Getter;

#[derive(Getter)]
struct Tuple(f64, (), #[get(name = "get", Const)] usize);

impl Tuple {
    // test that we indeed access the third field
    fn test_getter() {
        let t = Tuple(0_f64, (), 0_usize);
        assert_eq!(t.get(), &0_usize);
    }
}

// test the case of multiple field attribute on the same field
// see ../fail/get.rs for the fail test
#[derive(Getter)]
struct MultipleAttribute {
    #[get_mut]
    #[get(name = "field")]
    #[get(name = "get")]
    f: (),
    #[get]
    #[get_mut(name = "field_mut")]
    #[get_mut(name = "get_mut")]
    g: (),
}

impl MultipleAttribute {
    fn test_multiple_attr() {
        let mut m = Self { f: (), g: () };
        assert_eq!(m.get(), &());
        assert_eq!(m.f_mut(), &mut ());

        assert_eq!(m.get_mut(), &mut ());
        assert_eq!(m.g(), &());
    }
}

fn main() {
    Tuple::test_getter();
    MultipleAttribute::test_multiple_attr();
}
