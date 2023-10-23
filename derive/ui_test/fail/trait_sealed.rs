// fail test for argument in trait_sealed!() macro
use utils_lib_derive::trait_sealed;

trait_sealed!(arg);

fn main() {}
