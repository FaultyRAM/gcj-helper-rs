# gcj-helper

[![Travis](https://img.shields.io/travis/FaultyRAM/gcj-helper.svg)][1]
[![AppVeyor](https://img.shields.io/appveyor/ci/FaultyRAM/gcj-helper.svg)][2]
[![Crates.io](https://img.shields.io/crates/v/gcj-helper.svg)][3]
[![Docs.rs](https://docs.rs/gcj-helper/badge.svg)][4]

`gcj-helper` is a [Rust][5] library for writing [Google Code Jam][6] solutions. It handles the
boilerplate for you so you can focus on writing solutions instead.

## Usage

### Via `cargo new --template`

If you're creating a new crate, the quickest way to get started is to use the following command:

```text
cargo new --template https://github.com/FaultyRAM/gcj-template-rust.git foobar
```

This creates a new binary crate named `foobar` that is set up to use `gcj-helper`. No additional
work is needed; just open `src/main.rs` and start writing your solution.

### By hand

You can also manually add `gcj-helper` to your crate, though doing so is slower than using
`cargo new --template`. To do this, add the following line to your `[dependencies]` in `Cargo.toml`:

```toml
gcj-helper = "0.1"
```

And add the following line to your crate root:

```rust
extern crate gcj_helper;
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[1]: https://travis-ci.org/FaultyRAM/gcj-helper
[2]: https://ci.appveyor.com/project/FaultyRAM/gcj-helper
[3]: https://crates.io/crates/gcj-helper
[4]: https://docs.rs/gcj-helper
[5]: https://www.rust-lang.org
[6]: https://code.google.com/codejam/
