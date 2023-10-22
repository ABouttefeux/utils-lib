use std::sync::{Arc, Mutex};

use utils_lib_derive::Getter;

// First let us look at the base example without.
// The derive macro needs at lest one #[get] or #[get_mut] field attribute
// this will create the getter.
#[derive(Getter)]
struct Example1 {
    #[get] // this create an (immutable getter)
    #[get_mut] // this create a mutable getter
    field: Vec<u32>,
    field2: Vec<u32>,
}

// basically it would write the getter for [`Example1::field2`] as
impl Example1 {
    // for the #[get]
    fn field2(&self) -> &Vec<u32> {
        &self.field2
    }

    // and #[get_mut]
    fn field2_mut(&mut self) -> &mut Vec<u32> {
        &mut self.field2
    }
}

fn example_1() {
    let mut e = Example1 {
        field: vec![0, 1, 2, 3],
        field2: vec![0, 0, 0, 0],
    };
    assert_eq!(e.field(), &vec![0, 1, 2, 3]); // without name parameter it will be field() and return a reference
    assert_eq!(e.field_mut(), &mut vec![0, 1, 2, 3]); // and field_mut() and return a mut reference
    assert_eq!(e.field2(), &vec![0, 0, 0, 0]);
    assert_eq!(e.field2_mut(), &mut vec![0, 0, 0, 0]);
}

// However the name can be change using the `name` parameter
#[derive(Getter)]
struct ExampleName {
    #[get(name = "my_getter")] // it can be used name = "#"
    #[get_mut(name(my_getter_2))] // or name(#)
    field: u32,
}

// Note also that for tuple struct the name is a requirement
#[derive(Getter)]
struct TupleName(#[get(name = "get")] f32);

fn example_name() {
    let mut example = ExampleName { field: 1_u32 };
    assert_eq!(example.my_getter(), &1_u32);
    assert_eq!(example.my_getter_2(), &mut 1_u32);

    let example_tuple = TupleName(-1_f32);
    assert_eq!(example_tuple.get(), &-1_f32);
}

// The next option is the visibility
pub mod module {
    use super::*;

    // it is possible to modify the visibility of the getter using `visibility`.
    // By default it is private.
    // To see all the possibility please refer to the documentation of the derive macro.
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

// The following only apply to the immutable getter.
// The next option is the constance. By default the getter is not constant
#[derive(Getter)]
struct ExampleConst {
    #[get(Const)] // this create a constant getter
    field: Arc<Mutex<u32>>,
}

// this is constant
const fn constant_fn(e: &ExampleConst) -> &Arc<Mutex<u32>> {
    e.field() // this has to be constant otherwise it does not compile
}

fn example_const() {
    let e = ExampleConst {
        field: Arc::new(Mutex::new(0)),
    };
    assert_eq!(*constant_fn(&e).lock().unwrap(), 0);
}

// `getter_ty` option determine how the value is return. By default we return a reference.
// But it can be configure to return a copy or a clone of the field
#[derive(Getter)]
struct ExampleGetterTy {
    #[get(getter_ty = "copy")]
    field: u32,
    #[get(getter_ty = "clone")]
    field2: String,
}

fn example_getter_ty() {
    let e = ExampleGetterTy {
        field: 3_u32,
        field2: "this is another example".to_string(),
    };

    assert_eq!(e.field(), 3_u32);
    assert_eq!(e.field2(), "this is another example".to_string());
}

// the last option is `self_ty` that control the self type.
// By default it is by reference (meaning `&self`) but it can be change
// `by_value`` or `move` (meaning `self`). In the case it is require that
// `getter_ty` is set to `by_value`/`copy` (or `clone`)
#[derive(Getter)]
struct ExampleSelfTy {
    #[get(self_ty = "move", getter_ty = "by_value")]
    field: String,
}

fn example_self_ty() {
    let e = ExampleSelfTy {
        field: "Hello".to_string(),
    };

    assert_eq!(e.field(), "Hello".to_string());
    // from here e can't be use as it is doesn't implement `Copy`.
}

fn main() {
    example_1();
    example_name();
    example_visibility();
    example_const();
    example_getter_ty();
    example_self_ty();
}

#[cfg(test)]
#[test]
fn test() {
    main();
}
