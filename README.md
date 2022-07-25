# ⚾ #[catch_panic]

[![Crates.io](https://img.shields.io/crates/v/catch_panic.svg)](https://crates.io/crates/catch_panic)
[![Documentation](https://docs.rs/catch_panic/badge.svg)](https://docs.rs/catch_panic)
[![License](https://img.shields.io/crates/l/catch_panic)](https://github.com/HermitSocialClub/catch_panic)

A helper macro for safe Java-Rust interop that "catches" Rust panics and rethrows them as Java exceptions.


## Getting Started

Add `catch_panic` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
catch_panic = "1.0.0"
```


## Usage

// todo write example code


## License

This crate is dual-licensed under either:

- the [Apache License, Version 2.0](LICENSE-APACHE)
- the [MIT license](LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

The code in this crate is a derivative of code from [HermitSocialClub/HermitRepo][HermitRepo],
specifically the files [`ProjectTomato/tomato_macros/src/lib.rs`][tomato_macros_lib] and
[`ProjectTomato/tomato/src/util.rs`][tomato_util].
All authors of these two files have agreed to relicense the original code under the above license.

[HermitRepo]: https://github.com/HermitSocialClub/HermitRepo
[tomato_macros_lib]: https://github.com/HermitSocialClub/HermitRepo/blob/bedfbbd8ca332d473a4cde7b063d370990afd68f/ProjectTomato/tomato_macros/src/lib.rs
[tomato_util]: https://github.com/HermitSocialClub/HermitRepo/blob/bedfbbd8ca332d473a4cde7b063d370990afd68f/ProjectTomato/tomato/src/util.rs
