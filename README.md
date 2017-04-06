# gcj-helper

[![Travis](https://img.shields.io/travis/FaultyRAM/gcj-helper-rs.svg)][1]
[![AppVeyor](https://img.shields.io/appveyor/ci/FaultyRAM/gcj-helper-rs.svg)][2]
[![Crates.io](https://img.shields.io/crates/v/gcj-helper.svg)][3]
[![Docs.rs](https://docs.rs/gcj-helper/badge.svg)][4]

`gcj-helper` is a [Rust][5] library for writing [Google Code Jam][6] solutions. It handles the
usual boilerplate (opening files, reading/writing data, etc.), and optionally supports test case
parallelisation.

## Example

```rust
extern crate gcj_helper;

use gcj_helper::TestEngine;

fn main() {
    TestEngine::new("./foo.in", "./foo.out").run(
        |input| input.read_next_line().to_owned(),
        |data| format!(" {}\n", data),
    );
}
```

## Usage

### Via `cargo new --template`

For brand-new crates, the quickest way to get started is to use a Cargo template:

```text
cargo new --template https://github.com/FaultyRAM/gcj-template-rust.git foobar
```

This creates a new crate named `foobar` that is set up to use `gcj-helper`. No extra work is
needed; just open `src/main.rs` and start writing your solution.

### By hand

You can also manually add `gcj-helper` to your crate, though doing so is slower than using
`cargo new --template`. To do so, add this line to your `[dependencies]` in `Cargo.toml`:

```toml
gcj-helper = "0.5"
```

And add this line to your crate root:

```rust
extern crate gcj_helper;
```

### Test case parallelisation

By default, `gcj-helper` executes each test case in a single thread, one by one. If the `parallel`
feature is enabled, `gcj-helper` will attempt to execute multiple test cases simultaneously, but
this relies on third-party dependencies (currently [`rayon`][7]), resulting in slower build times.
If you'd like to enable this feature, open `Cargo.toml` and replace the following line:

```toml
gcj-helper = "0.5"
```

With this line:

```toml
gcj-helper = { version = "0.5", features = ["parallel"] }
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

[1]: https://travis-ci.org/FaultyRAM/gcj-helper-rs
[2]: https://ci.appveyor.com/project/FaultyRAM/gcj-helper-rs
[3]: https://crates.io/crates/gcj-helper
[4]: https://docs.rs/gcj-helper
[5]: https://www.rust-lang.org
[6]: https://code.google.com/codejam/
[7]: https://crates.io/crates/rayon
