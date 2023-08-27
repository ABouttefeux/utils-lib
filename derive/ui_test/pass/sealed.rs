mod private {
    pub trait Sealed {}
}

use utils_lib_derive::Sealed;

#[derive(Sealed)]
struct ZST;

#[derive(Sealed)]
struct S1 {}

#[derive(Sealed)]
struct S2 {
    a: Vec<S1>,
    g: u128,
    c: Box<S2>,
    d: *mut (),
    e: *mut *const usize,
}

#[derive(Copy, Sealed, Clone)]
struct S3;

// #[derive(Sealed)]
// struct S4<R: Clone> {
//     r: R,
// }

// #[derive(Sealed)]
// struct S5<'a> {
//     r: &'a mut S4,
// }

// #[derive(Sealed)]
// struct SlicePtr<'a, const SIZE: usize, T> {
//     ptr: &'a [T; SiZE],
// }

#[derive(Sealed)]
enum Name {
    A,
    B(bool, i8, i16, i32, i64),
    C { float: f64, s3: S3 },
    E,
    //F(S4<i32>),
    //G { a: S4<()> },
}

#[derive(Sealed)]
enum Never {}

// #[derive(Sealed)]
// enum MayBeSlice<'a, const SIZE: usize, T> {
//     Nope,
//     Yes(SlicePtr<'a, SIZE, T>),
// }

#[derive(Sealed)]
union IntOrFloat {
    i: u32,
    f: f32,
}

#[derive(Sealed)]
union LargePossibilities {
    array: [i128; 64],
    unit: (),
    i: i32,
    u: usize,
    ptr: *const LargePossibilities,
}

// #[derive(Sealed)]
// union RefOrNumber<'a, const SIZE: usize, T> {
//     prt: SlicePtr<'a, SIZE, T>,
//     n: usize,
// }

fn main() {}
