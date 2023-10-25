// pass test for trait_sealed!()
use utils_lib_derive::{trait_sealed, Sealed};

trait_sealed!();

#[derive(Sealed)]
struct S;

fn main() {}
