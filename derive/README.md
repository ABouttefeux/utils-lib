# Procedural macro: utils-lib-derive

[![Rust](https://github.com/ABouttefeux/utils-lib/actions/workflows/rust.yml/badge.svg?branch=develop)](https://github.com/ABouttefeux/utils-lib/actions/workflows/rust.yml)
![](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)
[![](https://img.shields.io/badge/doc-Read_Me-blueviolet)](https://abouttefeux.github.io/utils-lib/utils_lib_derive/index.html)
[![codecov](https://codecov.io/gh/ABouttefeux/utils-lib/branch/develop/graph/badge.svg?token=mUFucbIHuh)](https://codecov.io/gh/ABouttefeux/utils-lib)

Derive macro for getters and Sealed trait

# Example

## Getter

```rust
use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get(Pub, Const)]
    f: u32,
    #[get_mut(Pub)]
    f2: i8
}

fn main() {
    let mut s = S { f: 1_u32, f2: -1_i8 };
    assert_eq!(s.f(), &1_u32);
    assert_eq!(s.f2_mut(), &-1_i8);
}
```

## Sealed

```rust
use utils_lib_derive::{trait_sealed, Sealed};

// this create a module named [`private`] with a trait named [`Sealed`]
// without method inside that module.
trait_sealed!();

#[derive(Sealed)]
struct S;

/// this trait is sealed and cannot me implemented outside of this crate
/// because [`Sealed`] is a private trait that can't be implemented outside
/// of this crate.
pub trait Trait: private::Sealed {}

impl Trait for S {}

fn main() {}
```
[See more example](https://github.com/ABouttefeux/utils-lib/tree/main/derive/examples)