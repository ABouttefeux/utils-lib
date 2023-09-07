# utils-lib

<!-- [![Build Status](https://drone.noxie.ch/api/badges/ABouttefeux/utils_lib/status.svg)](https://drone.noxie.ch/ABouttefeux/utils_lib) -->
[![Rust](https://github.com/ABouttefeux/utils-lib/actions/workflows/rust.yml/badge.svg?branch=develop)](https://github.com/ABouttefeux/utils-lib/actions/workflows/rust.yml)
![](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)
[![](https://img.shields.io/badge/doc-Read_Me-blueviolet)](https://abouttefeux.github.io/utils-lib/utils_lib/index.html)
[![codecov](https://codecov.io/gh/ABouttefeux/utils-lib/branch/develop/graph/badge.svg?token=mUFucbIHuh)](https://codecov.io/gh/ABouttefeux/utils-lib)

A bunch of utilities that I uses across several project. 
Instead of copying the same code everywhere I finally decided to put everything into a library. 
I will probably continue grow this library.

I won't maintain this crate actively and I might introduce breaking change at anytime. 
I also might be slow to merge the changes on the main branch.
Also I won't publish this crate on crates.io so you will have to specify the git url in the cargo.toml file.
```toml
[dependencies]
utils-lib = { version = "0.1.0", git = "https://git.noxie.ch/ABouttefeux/utils-lib" }
```

If you want the bleeding edge feature you might want to target the develop branch using
```toml
[dependencies]
utils-lib = { git = "https://git.noxie.ch/ABouttefeux/utils-lib", branch = "develop" }
```
again it might break at anytime so you might want to target an specific commit
```toml
[dependencies]
utils-lib = { git = "https://git.noxie.ch/ABouttefeux/utils-lib", branch = "develop", rev="<commit hash>" }
```
