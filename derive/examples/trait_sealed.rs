use utils_lib_derive::{trait_sealed, Sealed};

use crate::private::Sealed;

// this create a module [`private`] and a private trait Sealed without method
// inside that module.
trait_sealed!();

/// this trait is sealed and cannot me implemented outside of this crate
/// because [`Sealed`] is a private trait that can't be implemented outside
/// of this crate.
pub trait SealedTrait: Sealed {}

#[derive(Sealed)]
struct S;

impl SealedTrait for S {}

fn main() {}

#[cfg(test)]
#[test]
fn test() {
    main();
}
