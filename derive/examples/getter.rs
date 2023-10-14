use utils_lib_derive::Getter;

/// First let us look at the base example without.
/// The derive macro needs at lest one #[get] or #[get_mut] field attribute
/// this will create the getter.
#[derive(Getter)]
struct Example1 {
    #[get] // this create an (immutable getter)
    #[get_mut] // this create a mutable getter
    field: u32,
}

fn example_1() {
    let mut e = Example1 { field: 0_u32 };
    assert_eq!(e.field(), &0); // without name parameter it will be field() and return a reference
    assert_eq!(e.field_mut(), &mut 0); // and field_mut() and return a mut reference
}

/// However the name can be change using the `name` parameter
#[derive(Getter)]
struct ExampleName {
    #[get(name = "my_getter")] // it can be used name = "#"
    #[get_mut(name(my_getter_2))] // or name(#)
    field: u32,
}

/// Note also that for tuple struct the name is a requirement
#[derive(Getter)]
struct TupleName(#[get(name = "get")] f32);

fn example_name() {
    let mut example = ExampleName { field: 1_u32 };
    assert_eq!(example.my_getter(), &1_u32);
    assert_eq!(example.my_getter_2(), &mut 1_u32);

    let example_tuple = TupleName(-1_f32);
    assert_eq!(example_tuple.get(), &-1_f32);
}

/// The next option is the visibility
pub mod module {
    use super::*;

    /// it is possible to modify the visibility of the getter using `visibility`.
    /// By default it is private.
    /// To see all the possibility please refer to the documentation of the derive macro.
    #[derive(Getter)]
    pub struct ExampleVisibility {
        #[get(Pub)]
        #[get_mut(visibility = "pub(crate)")]
        pub field: u32,
    }
}

fn example_visibility() {
    let mut e = module::ExampleVisibility { field: 2_u32 };

    assert_eq!(e.field(), &2_u32);
    assert_eq!(e.field_mut(), &mut 2_u32);
}

// the following only apply to the immutable getter

fn main() {
    example_1();
    example_name();
    example_visibility()
}

#[cfg(test)]
#[test]
fn test() {
    main();
}
